use crate::{mock_write::MockWrite, *};

/// Run the program and return its TerminationInfo, using default memory params.
/// Stdout/stderr are just forwarded to the host.
pub fn run_program<M: Memory>(prog: Program) -> TerminationInfo {
    run_program_with_config::<M>(prog, Default::default())
}

/// Like [`run_program`], but accepts explicit memory parameters.
/// Stdout/stderr are just forwarded to the host.
pub fn run_program_with_config<M: Memory>(prog: Program, params: M::Params) -> TerminationInfo {
    let out = std::io::stdout();
    let err = std::io::stderr();

    let res: Result<!, TerminationInfo> = run::<M>(prog, params, out, err);
    match res {
        Ok(never) => never,
        Err(t) => t,
    }
}

/// Run the program and return stdout as a `Vec<String>`, using default memory params,
/// or a termination info if it did not terminate correctly. Stderr is just forwarded to the host.
pub fn get_stdout<M: Memory>(prog: Program) -> Result<Vec<String>, TerminationInfo> {
    let out = MockWrite::new();
    let err = std::io::stderr();

    let res = run::<M>(prog, Default::default(), out.clone(), err);
    match res {
        Ok(never) => never,
        Err(TerminationInfo::MachineStop) => Ok(out.into_strings()),
        Err(info) => Err(info),
    }
}

/// Run the program to completion using the given writers for stdout/stderr.
///
/// We fix `BasicMemory` as a memory for now.
fn run<M: Memory>(
    prog: Program,
    params: M::Params,
    stdout: impl GcWrite,
    stderr: impl GcWrite,
) -> Result<!, TerminationInfo> {
    let res: NdResult<!> = try {
        let mut machine =
            Machine::<M>::new(prog, params, DynWrite::new(stdout), DynWrite::new(stderr))?;

        loop {
            machine.step()?;

            // Drops everything not reachable from `machine`.
            mark_and_sweep(&machine);
        }
    };

    // Extract the TerminationInfo from the `NdResult<!>`.
    res.get_internal()
}
