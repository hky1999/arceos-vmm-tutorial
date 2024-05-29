use super::hal::AxvmHalImpl;
use axerrno::AxResult;
use axvm::arch::VmxExitReason;
use axvm::AxvmVcpu;

type Vcpu = AxvmVcpu<AxvmHalImpl>;

const VM_EXIT_INSTR_LEN_VMCALL: u8 = 3;

fn handle_hypercall(vcpu: &mut Vcpu) -> AxResult {
    let regs = vcpu.regs();
    info!(
        "VM exit: VMCALL({:#x}): {:?}",
        regs.rax,
        [regs.rdi, regs.rsi, regs.rdx, regs.rcx]
    );
    vcpu.advance_rip(VM_EXIT_INSTR_LEN_VMCALL)?;
    Ok(())
}

pub fn vmexit_handler(vcpu: &mut Vcpu) -> AxResult {
    let exit_info = vcpu.exit_info()?;
    debug!("VM exit: {:#x?}", exit_info);

    if exit_info.entry_failure {
        panic!("VM entry failed: {:#x?}", exit_info);
    }

    let res = match exit_info.exit_reason {
        VmxExitReason::VMCALL => handle_hypercall(vcpu),
        _ => panic!(
            "Unhandled VM-Exit reason {:?}:\n{:#x?}",
            exit_info.exit_reason, vcpu
        ),
    };

    if res.is_err() {
        panic!(
            "Failed to handle VM-exit {:?}:\n{:#x?}",
            exit_info.exit_reason, vcpu
        );
    }

    Ok(())
}