use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    back::{
        self,
        hlsl::{
            writer::{EntryPointBinding, EpStructMember, Io},
            BackendResult, Error,
        },
    },
    proc::NameKey,
    Handle, Module, ShaderStage, TypeInner,
};

impl<W: core::fmt::Write> super::Writer<'_, W> {
    /// Mesh and task entry points must all return at the same `return` statement,
    /// so we have a nested function that can return wherever. This writes the caller,
    /// or the actual entry point.
    #[expect(clippy::too_many_arguments)]
    pub(super) fn write_nested_function_outer(
        &mut self,
        module: &Module,
        func_ctx: &back::FunctionCtx,
        header: &str,
        name: &str,
        need_workgroup_variables_initialization: bool,
        nested_name: &str,
        entry_point: &crate::EntryPoint,
    ) -> BackendResult {
        let mut any_args_written = false;
        let mut separator = || {
            if any_args_written {
                ", "
            } else {
                any_args_written = true;
                ""
            }
        };

        let back::FunctionType::EntryPoint(ep_index) = func_ctx.ty else {
            unreachable!();
        };
        let stage = module.entry_points[ep_index as usize].stage;
        write!(self.out, "{header}")?;
        write!(self.out, "void {name}(")?;
        let mut arg_names = Vec::new();
        if let Some(ref ep_input) = self.entry_point_io.get(&(ep_index as usize)).unwrap().input {
            write!(self.out, "{} {}", ep_input.ty_name, ep_input.arg_name)?;
            arg_names.push(ep_input.arg_name.clone());
        } else {
            for (index, arg) in entry_point.function.arguments.iter().enumerate() {
                write!(self.out, "{}", separator())?;
                self.write_type(module, arg.ty)?;

                let argument_name =
                    &self.names[&NameKey::EntryPointArgument(ep_index, index as u32)];
                arg_names.push(argument_name.clone());

                write!(self.out, " {argument_name}")?;
                if let TypeInner::Array { base, size, .. } = module.types[arg.ty].inner {
                    self.write_array_size(module, base, size)?;
                }

                self.write_semantic(&arg.binding, Some((stage, Io::Input)))?;
            }
        }
        if need_workgroup_variables_initialization || stage == ShaderStage::Mesh {
            write!(
                self.out,
                "{}uint __local_invocation_index : SV_GroupIndex",
                separator()
            )?;
        }
        if let Some(ref mesh_info) = entry_point.mesh_info {
            // Mesh shader wrapper
            let mesh_interface = self.entry_point_io.get(&(ep_index as usize)).unwrap();
            let vert_info = mesh_interface.mesh_vertices.as_ref().unwrap();
            let prim_info = mesh_interface.mesh_primitives.as_ref().unwrap();
            let indices_info = mesh_interface.mesh_indices.as_ref().unwrap();
            write!(
                self.out,
                "{}out indices {} {}[{}]",
                separator(),
                indices_info.ty_name,
                indices_info.arg_name,
                mesh_info.max_primitives
            )?;
            write!(
                self.out,
                ", out vertices {} {}[{}]",
                vert_info.ty_name, vert_info.arg_name, mesh_info.max_vertices
            )?;
            write!(
                self.out,
                ", out primitives {} {}[{}]",
                prim_info.ty_name, prim_info.arg_name, mesh_info.max_primitives
            )?;
            if let Some(task_payload) = entry_point.task_payload {
                // Set task payload variable
                write!(self.out, ", in payload ")?;
                let var = &module.global_variables[task_payload];
                self.write_type(module, var.ty)?;

                let name = &self.names[&NameKey::GlobalVariable(task_payload)];
                write!(self.out, " {name}")?;
                arg_names.push(name.clone());
                if let TypeInner::Array { base, size, .. } = module.types[var.ty].inner {
                    self.write_array_size(module, base, size)?;
                }
            }
            writeln!(self.out, ") {{")?;
            if need_workgroup_variables_initialization {
                writeln!(
                    self.out,
                    "{}if (all(__local_invocation_index == 0)) {{",
                    back::INDENT
                )?;
                self.write_workgroup_variables_initialization(
                    func_ctx,
                    module,
                    module.entry_points[ep_index as usize].stage,
                )?;
                writeln!(self.out, "{}}}", back::INDENT)?;
                self.write_control_barrier(crate::Barrier::WORK_GROUP, back::Level(1))?;
            }
            write!(self.out, "{}{nested_name}(", back::INDENT)?;
            for (i, arg_name) in arg_names.iter().enumerate() {
                if i != 0 {
                    write!(self.out, ", ")?;
                }
                write!(self.out, "{arg_name}")?;
            }
            writeln!(self.out, ");")?;
            writeln!(
                self.out,
                "{}GroupMemoryBarrierWithGroupSync();",
                back::INDENT
            )?;

            let back::FunctionType::EntryPoint(ep_idx) = func_ctx.ty else {
                unreachable!()
            };
            let ep = &module.entry_points[ep_idx as usize];
            let mesh_info = ep.mesh_info.as_ref().unwrap();
            let io = self.entry_point_io.get(&(ep_idx as usize)).unwrap();

            let var_name = &self.names[&NameKey::GlobalVariable(mesh_info.output_variable)];
            let var_type = module.global_variables[mesh_info.output_variable].ty;
            let wg_size: u32 = ep.workgroup_size.iter().product();

            let get_var_member_name = |bi, var_type| {
                let TypeInner::Struct { ref members, .. } = module.types[var_type].inner else {
                    unreachable!()
                };
                let idx = members
                    .iter()
                    .position(|f| f.binding == Some(crate::Binding::BuiltIn(bi)))
                    .unwrap();
                self.names[&NameKey::StructMember(var_type, idx as u32)].clone()
            };

            let vert_count = format!(
                "{var_name}.{}",
                get_var_member_name(crate::BuiltIn::VertexCount, var_type),
            );
            let prim_count = format!(
                "{var_name}.{}",
                get_var_member_name(crate::BuiltIn::PrimitiveCount, var_type),
            );

            let level = back::Level(1);

            writeln!(
                self.out,
                "{level}SetMeshOutputCounts({vert_count}, {prim_count});"
            )?;

            // We need separate loops for vertices and primitives writing
            struct OutputArray<'a> {
                array_bi: crate::BuiltIn,
                count: String,
                io_interface: &'a EntryPointBinding,
                is_primitive: bool,
                index_name: &'static str,
                ty: Handle<crate::Type>,
            }
            let output_arrays = [
                OutputArray {
                    array_bi: crate::BuiltIn::Vertices,
                    count: vert_count,
                    io_interface: io.mesh_vertices.as_ref().unwrap(),
                    is_primitive: false,
                    index_name: "vertIndex",
                    ty: mesh_info.vertex_output_type,
                },
                OutputArray {
                    array_bi: crate::BuiltIn::Primitives,
                    count: prim_count,
                    io_interface: io.mesh_primitives.as_ref().unwrap(),
                    is_primitive: true,
                    index_name: "primIndex",
                    ty: mesh_info.primitive_output_type,
                },
            ];

            for output in output_arrays {
                let OutputArray {
                    array_bi,
                    count,
                    io_interface,
                    is_primitive,
                    index_name,
                    ty,
                } = output;
                let out_var_name = &io_interface.arg_name;
                let index_name = self.namer.call(index_name);
                let array_name = get_var_member_name(array_bi, var_type);
                let item_name = format!("{var_name}.{array_name}[{index_name}]");
                writeln!(
                    self.out,
                    "{level}for (int {index_name} = __local_invocation_index; {index_name} < {count}; {index_name} += {}) {{",
                    wg_size
                )?;

                // Loop body, uses more indentation
                {
                    let level = level.next();
                    for member in &io_interface.members {
                        let out_member_name = &member.name;
                        let in_member_name = &self.names[&NameKey::StructMember(ty, member.index)];
                        writeln!(self.out, "{level}{out_var_name}[{index_name}].{out_member_name} = {item_name}.{in_member_name};",)?;
                    }
                    if is_primitive {
                        let indices_member_name = get_var_member_name(
                            mesh_info.topology.to_builtin(),
                            mesh_info.primitive_output_type,
                        );
                        let indices_var_name = &io.mesh_indices.as_ref().unwrap().arg_name;
                        writeln!(
                                self.out,
                                "{level}{indices_var_name}[{index_name}] = {item_name}.{indices_member_name};",
                            )?;
                    }
                }

                writeln!(self.out, "{level}}}")?;
            }
            writeln!(self.out, "}}")?;
        } else {
            // Task shader wrapper
            writeln!(self.out, ") {{")?;
            if need_workgroup_variables_initialization {
                writeln!(
                    self.out,
                    "{}if (all(__local_invocation_index == 0)) {{",
                    back::INDENT
                )?;
                self.write_workgroup_variables_initialization(
                    func_ctx,
                    module,
                    module.entry_points[ep_index as usize].stage,
                )?;
                writeln!(self.out, "{}}}", back::INDENT)?;
                self.write_control_barrier(crate::Barrier::WORK_GROUP, back::Level(1))?;
            }
            let grid_size = self.namer.call("gridSize");
            write!(
                self.out,
                "{}uint3 {grid_size} = {nested_name}(",
                back::INDENT
            )?;
            for (i, arg_name) in arg_names.iter().enumerate() {
                if i != 0 {
                    write!(self.out, ", ")?;
                }
                write!(self.out, "{arg_name}")?;
            }
            writeln!(self.out, ");")?;
            writeln!(
                self.out,
                "{}GroupMemoryBarrierWithGroupSync();",
                back::INDENT
            )?;
            if let Some(limits) = self.options.task_runtime_limits {
                let level = back::Level(2);
                writeln!(self.out, "{}if (", back::INDENT)?;

                let max_per_dim = limits.max_mesh_workgroups_per_dim.min(2 << 21);
                let max_total = limits.max_mesh_workgroups_total;
                for i in 0..3 {
                    writeln!(
                        self.out,
                        "{level}{grid_size}.{} > {max_per_dim} ||",
                        back::COMPONENTS[i],
                    )?;
                }
                writeln!(
                    self.out,
                    "{level}((uint64_t){grid_size}.x) * ((uint64_t){grid_size}.y) * ((uint64_t){grid_size}.z) > {max_total}",
                )?;

                writeln!(self.out, "{}) {{", back::INDENT)?;
                writeln!(self.out, "{level}{grid_size} = uint3(0, 0, 0);")?;
                writeln!(self.out, "{}}}", back::INDENT)?;
            }
            writeln!(
                self.out,
                "{}DispatchMesh({grid_size}.x, {grid_size}.y, {grid_size}.z, {});",
                back::INDENT,
                self.names[&NameKey::GlobalVariable(entry_point.task_payload.unwrap())]
            )?;
            writeln!(self.out, "}}")?;
        }

        Ok(())
    }

    pub(super) fn write_ep_mesh_output_struct(
        &mut self,
        module: &Module,
        entry_point_name: &str,
        is_primitive: bool,
        mesh_info: &crate::MeshStageInfo,
    ) -> Result<EntryPointBinding, Error> {
        let (in_type, io, var_prefix, arg_name) = if is_primitive {
            (
                mesh_info.primitive_output_type,
                Io::MeshPrimitives,
                "Primitive",
                "primitives",
            )
        } else {
            (
                mesh_info.vertex_output_type,
                Io::MeshVertices,
                "Vertex",
                "vertices",
            )
        };
        let struct_name = format!("Mesh{var_prefix}Output_{entry_point_name}",);

        let members = match module.types[in_type].inner {
            TypeInner::Struct { ref members, .. } => members,
            _ => unreachable!(),
        };
        let mut out_members = Vec::new();
        for (index, member) in members.iter().enumerate() {
            if matches!(
                member.binding,
                Some(crate::Binding::BuiltIn(
                    crate::BuiltIn::PointIndex
                        | crate::BuiltIn::LineIndices
                        | crate::BuiltIn::TriangleIndices
                ))
            ) {
                continue;
            }
            let member_name = self.namer.call_or(&member.name, "member");
            out_members.push(EpStructMember {
                name: member_name,
                ty: member.ty,
                binding: member.binding.clone(),
                index: index as u32,
            })
        }
        self.write_interface_struct(
            module,
            (ShaderStage::Mesh, io),
            struct_name,
            Some(arg_name),
            out_members,
        )
    }

    pub(super) fn write_ep_mesh_output_indices(
        &mut self,
        topology: crate::MeshOutputTopology,
    ) -> Result<EntryPointBinding, Error> {
        let (indices_name, indices_type) = match topology {
            crate::MeshOutputTopology::Points => unreachable!(),
            crate::MeshOutputTopology::Lines => (self.namer.call("lineIndices"), "uint2"),
            crate::MeshOutputTopology::Triangles => (self.namer.call("triangleIndices"), "uint3"),
        };
        Ok(EntryPointBinding {
            ty_name: indices_type.to_string(),
            arg_name: indices_name,
            members: Vec::new(),
        })
    }
}
