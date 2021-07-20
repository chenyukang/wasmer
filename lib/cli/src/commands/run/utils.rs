use anyhow::Result;
use wasmer::*;

pub fn mapping_function_args(func: &Function, args: &[String]) -> Result<Vec<Value>> {
    let func_ty = func.ty();
    let required_arguments = func_ty.params().len();
    let provided_arguments = args.len();
    if required_arguments != provided_arguments {
        bail!(
            "Function expected {} arguments, but received {}: \"{}\"",
            required_arguments,
            provided_arguments,
            args.join(" ")
        );
    }
    let arg_vals = args
        .iter()
        .zip(func_ty.params().iter())
        .map(|(arg, param_type)| match param_type {
            ValType::I32 => {
                Ok(Val::I32(arg.parse().map_err(|_| {
                    anyhow!("Can't convert `{}` into a i32", arg)
                })?))
            }
            ValType::I64 => {
                Ok(Val::I64(arg.parse().map_err(|_| {
                    anyhow!("Can't convert `{}` into a i64", arg)
                })?))
            }
            ValType::F32 => {
                Ok(Val::F32(arg.parse().map_err(|_| {
                    anyhow!("Can't convert `{}` into a f32", arg)
                })?))
            }
            ValType::F64 => {
                Ok(Val::F64(arg.parse().map_err(|_| {
                    anyhow!("Can't convert `{}` into a f64", arg)
                })?))
            }
            _ => Err(anyhow!(
                "Don't know how to convert {} into {:?}",
                arg,
                param_type
            )),
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(arg_vals)
}
