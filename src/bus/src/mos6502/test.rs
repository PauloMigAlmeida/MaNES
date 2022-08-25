use super::*;

#[test]
fn test_set_flag() {
    let mut cpu = Mos6502::new();
    cpu.flags = 0;

    cpu.set_flag(Carry);
    assert_eq!(cpu.flags, 0b0000_0001);
    cpu.set_flag(Zero);
    assert_eq!(cpu.flags, 0b0000_0011);
    cpu.set_flag(DisableInterrupt);
    assert_eq!(cpu.flags, 0b0000_0111);
    cpu.set_flag(Decimal);
    assert_eq!(cpu.flags, 0b0000_1111);
    cpu.set_flag(Break);
    assert_eq!(cpu.flags, 0b0001_1111);
    cpu.set_flag(Overflow);
    assert_eq!(cpu.flags, 0b0101_1111);
    cpu.set_flag(Negative);
    assert_eq!(cpu.flags, 0b1101_1111);
}

#[test]
fn test_clear_flag() {
    let mut cpu = Mos6502::new();
    cpu.flags = 0b1101_1111;

    cpu.clear_flag(Carry);
    assert_eq!(cpu.flags, 0b1101_1110);
    cpu.clear_flag(Zero);
    assert_eq!(cpu.flags, 0b1101_1100);
    cpu.clear_flag(DisableInterrupt);
    assert_eq!(cpu.flags, 0b1101_1000);
    cpu.clear_flag(Decimal);
    assert_eq!(cpu.flags, 0b1101_0000);
    cpu.clear_flag(Break);
    assert_eq!(cpu.flags, 0b1100_0000);
    cpu.clear_flag(Overflow);
    assert_eq!(cpu.flags, 0b1000_0000);
    cpu.clear_flag(Negative);
    assert_eq!(cpu.flags, 0b0000_0000);
}

#[test]
fn test_stack_push() {
    let mut cpu = Mos6502::new();
    let mut bus = Bus::new();

    cpu.sp = 0xff;
    cpu.stack_push(0x10, &mut bus);
    assert_eq!(cpu.sp, 0xfe);
    cpu.stack_push(0x11, &mut bus);
    assert_eq!(cpu.sp, 0xfd);
    assert_eq!(bus.cpu_read_u8(STACK_PAGE | 0xff, false), 0x10);
    assert_eq!(bus.cpu_read_u8(STACK_PAGE | 0xfe, false), 0x11);
}

#[test]
#[should_panic]
fn test_stack_push_overflow() {
    let mut cpu = Mos6502::new();
    let mut bus = Bus::new();

    cpu.sp = 0x00;
    cpu.stack_push(0x10, &mut bus);
}

#[test]
fn test_stack_pull() {
    let mut cpu = Mos6502::new();
    let mut bus = Bus::new();

    cpu.sp = 0xff;
    cpu.stack_push(0x10, &mut bus);
    cpu.stack_push(0x11, &mut bus);
    assert_eq!(cpu.stack_pull(&bus), 0x11);
    assert_eq!(cpu.sp, 0xfe);
    assert_eq!(cpu.stack_pull(&bus), 0x10);
    assert_eq!(cpu.sp, 0xff);
}

#[test]
#[should_panic]
fn test_stack_underflow() {
    let mut cpu = Mos6502::new();
    let bus = Bus::new();

    cpu.sp = 0xff;
    cpu.stack_pull(&bus);
}

// TODO implement vectors otherwise this test will fail
#[test]
#[ignore]
fn test_cpu_reset() {
    let mut cpu = Mos6502::new();
    let mut bus = Bus::new();

    bus.cpu_write_u16(0xFFFC, 0x1234);
    cpu.a = 0x1;
    cpu.x = 0x1;
    cpu.y = 0x1;
    cpu.sp = 0xC0;
    cpu.flags = 0xFF;

    cpu.reset(&bus);
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.x, 0);
    assert_eq!(cpu.y, 0);
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.flags, 0b0010_0000);
    assert_eq!(cpu.cycles, 8);
}