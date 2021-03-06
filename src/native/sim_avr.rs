extern crate va_list;
/* automatically generated by rust-bindgen */

use native::stdint::*;
pub type avr_irq_notify_t =
    ::std::option::Option<unsafe extern "C" fn(irq: *mut Struct_avr_irq_t,
                                               value: uint32_t,
                                               param:
                                                   *mut ::std::os::raw::c_void)>;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed1 {
    IRQ_FLAG_NOT = 1,
    IRQ_FLAG_FILTERED = 2,
    IRQ_FLAG_ALLOC = 4,
    IRQ_FLAG_INIT = 8,
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_irq_pool_t {
    pub count: ::std::os::raw::c_int,
    pub irq: *mut *mut Struct_avr_irq_t,
}
impl ::std::clone::Clone for Struct_avr_irq_pool_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_irq_pool_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_irq_pool_t = Struct_avr_irq_pool_t;
pub enum Struct_avr_irq_hook_t { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_irq_t {
    pub pool: *mut Struct_avr_irq_pool_t,
    pub name: *const ::std::os::raw::c_char,
    pub irq: uint32_t,
    pub value: uint32_t,
    pub flags: uint8_t,
    pub hook: *mut Struct_avr_irq_hook_t,
}
impl ::std::clone::Clone for Struct_avr_irq_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_irq_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_irq_t = Struct_avr_irq_t;
pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type imaxdiv_t = Struct_Unnamed2;
pub type avr_cycle_count_t = uint64_t;
pub type avr_io_addr_t = uint16_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_regbit_t {
    pub _bindgen_bitfield_1_: uint32_t,
    pub _bindgen_bitfield_2_: uint32_t,
    pub _bindgen_bitfield_3_: uint32_t,
}
impl ::std::clone::Clone for Struct_avr_regbit_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_regbit_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_regbit_t = Struct_avr_regbit_t;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed3 {
    AVR_INT_IRQ_PENDING = 0,
    AVR_INT_IRQ_RUNNING = 1,
    AVR_INT_IRQ_COUNT = 2,
    AVR_INT_ANY = 255,
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_int_vector_t {
    pub vector: uint8_t,
    pub enable: avr_regbit_t,
    pub raised: avr_regbit_t,
    pub irq: [avr_irq_t; 2usize],
    pub _bindgen_bitfield_1_: uint8_t,
    pub _bindgen_bitfield_2_: uint8_t,
    pub _bindgen_bitfield_3_: uint8_t,
}
impl ::std::clone::Clone for Struct_avr_int_vector_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_int_vector_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_int_vector_t = Struct_avr_int_vector_t;
pub type avr_int_vector_p = *mut Struct_avr_int_vector_t;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed4 { avr_int_pending_overflow_f = 1, }
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed5 { avr_int_pending_fifo_size = 64, }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_int_pending_t {
    pub buffer: [avr_int_vector_p; 64usize],
    pub read: uint16_t,
    pub write: uint16_t,
    pub flags: uint8_t,
}
impl ::std::clone::Clone for Struct_avr_int_pending_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_int_pending_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_int_pending_t = Struct_avr_int_pending_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_int_table_t {
    pub vector: [*mut avr_int_vector_t; 64usize],
    pub vector_count: uint8_t,
    pub pending: avr_int_pending_t,
    pub running_ptr: uint8_t,
    pub running: [*mut avr_int_vector_t; 64usize],
    pub irq: [avr_irq_t; 2usize],
}
impl ::std::clone::Clone for Struct_avr_int_table_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_int_table_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_int_table_t = Struct_avr_int_table_t;
pub type avr_int_table_p = *mut Struct_avr_int_table_t;
pub type avr_cycle_timer_t =
    ::std::option::Option<unsafe extern "C" fn(avr: *mut Struct_avr_t,
                                               when: avr_cycle_count_t,
                                               param:
                                                   *mut ::std::os::raw::c_void)
                              -> avr_cycle_count_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_cycle_timer_slot_t {
    pub next: *mut Struct_avr_cycle_timer_slot_t,
    pub when: avr_cycle_count_t,
    pub timer: avr_cycle_timer_t,
    pub param: *mut ::std::os::raw::c_void,
}
impl ::std::clone::Clone for Struct_avr_cycle_timer_slot_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_cycle_timer_slot_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_cycle_timer_slot_t = Struct_avr_cycle_timer_slot_t;
pub type avr_cycle_timer_slot_p = *mut Struct_avr_cycle_timer_slot_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_cycle_timer_pool_t {
    pub timer_slots: [avr_cycle_timer_slot_t; 64usize],
    pub timer_free: avr_cycle_timer_slot_p,
    pub timer: avr_cycle_timer_slot_p,
}
impl ::std::clone::Clone for Struct_avr_cycle_timer_pool_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_cycle_timer_pool_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_cycle_timer_pool_t = Struct_avr_cycle_timer_pool_t;
pub type avr_cycle_timer_pool_p = *mut Struct_avr_cycle_timer_pool_t;
pub type avr_flashaddr_t = uint32_t;
pub type avr_io_read_t =
    ::std::option::Option<unsafe extern "C" fn(avr: *mut Struct_avr_t,
                                               addr: avr_io_addr_t,
                                               param:
                                                   *mut ::std::os::raw::c_void)
                              -> uint8_t>;
pub type avr_io_write_t =
    ::std::option::Option<unsafe extern "C" fn(avr: *mut Struct_avr_t,
                                               addr: avr_io_addr_t,
                                               v: uint8_t,
                                               param:
                                                   *mut ::std::os::raw::c_void)>;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed6 {
    S_C = 0,
    S_Z = 1,
    S_N = 2,
    S_V = 3,
    S_S = 4,
    S_H = 5,
    S_T = 6,
    S_I = 7,
    R_XL = 26,
    R_XH = 27,
    R_YL = 28,
    R_YH = 29,
    R_ZL = 30,
    R_ZH = 31,
    R_SPL = 93,
    R_SPH = 94,
    R_SREG = 95,
    MAX_IOs = 280,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed7 {
    LOG_OUTPUT = 0,
    LOG_ERROR = 1,
    LOG_WARNING = 2,
    LOG_TRACE = 3,
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CPUState {
    cpu_Limbo = 0,
    cpu_Stopped = 1,
    cpu_Running = 2,
    cpu_Sleeping = 3,
    cpu_Step = 4,
    cpu_StepDone = 5,
    cpu_Done = 6,
    cpu_Crashed = 7,
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_trace_data_t {
    pub codeline: *mut *mut Struct_avr_symbol_t,
    pub old: [Struct_Unnamed9; 32usize],
    pub old_pci: ::std::os::raw::c_int,
    pub touched: [uint32_t; 8usize],
}
impl ::std::clone::Clone for Struct_avr_trace_data_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_trace_data_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub pc: uint32_t,
    pub sp: uint16_t,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_run_t =
    ::std::option::Option<unsafe extern "C" fn(avr: *mut Struct_avr_t)>;
pub enum Struct_avr_vcd_t { }
pub enum Struct_avr_gdb_t { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_t {
    pub mmcu: *const ::std::os::raw::c_char,
    pub ramend: uint16_t,
    pub flashend: uint32_t,
    pub e2end: uint32_t,
    pub vector_size: uint8_t,
    pub signature: [uint8_t; 3usize],
    pub fuse: [uint8_t; 4usize],
    pub rampz: avr_io_addr_t,
    pub eind: avr_io_addr_t,
    pub address_size: uint8_t,
    pub codeend: uint32_t,
    pub state: ::std::os::raw::c_int,
    pub frequency: uint32_t,
    pub vcc: uint32_t,
    pub avcc: uint32_t,
    pub aref: uint32_t,
    pub cycle: avr_cycle_count_t,
    pub run_cycle_count: avr_cycle_count_t,
    pub run_cycle_limit: avr_cycle_count_t,
    pub sleep_usec: uint32_t,
    pub init: ::std::option::Option<unsafe extern "C" fn(avr:
                                                             *mut Struct_avr_t)>,
    pub reset: ::std::option::Option<unsafe extern "C" fn(avr:
                                                              *mut Struct_avr_t)>,
    pub custom: Struct_Unnamed10,
    pub run: avr_run_t,
    pub sleep: ::std::option::Option<unsafe extern "C" fn(avr:
                                                              *mut Struct_avr_t,
                                                          howLong:
                                                              avr_cycle_count_t)>,
    pub irq_pool: avr_irq_pool_t,
    pub sreg: [uint8_t; 8usize],
    pub interrupt_state: int8_t,
    pub pc: avr_flashaddr_t,
    pub reset_pc: avr_flashaddr_t,
    pub io: [Struct_Unnamed11; 280usize],
    pub io_shared_io_count: ::std::os::raw::c_int,
    pub io_shared_io: [Struct_Unnamed14; 4usize],
    pub flash: *mut uint8_t,
    pub data: *mut uint8_t,
    pub io_port: *mut Struct_avr_io_t,
    pub cycle_timers: avr_cycle_timer_pool_t,
    pub interrupts: avr_int_table_t,
    pub _bindgen_bitfield_1_: uint8_t,
    pub _bindgen_bitfield_2_: uint8_t,
    pub trace_data: *mut Struct_avr_trace_data_t,
    pub vcd: *mut Struct_avr_vcd_t,
    pub gdb: *mut Struct_avr_gdb_t,
    pub gdb_port: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_avr_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub init: ::std::option::Option<unsafe extern "C" fn(avr:
                                                             *mut Struct_avr_t,
                                                         data:
                                                             *mut ::std::os::raw::c_void)>,
    pub deinit: ::std::option::Option<unsafe extern "C" fn(avr:
                                                               *mut Struct_avr_t,
                                                           data:
                                                               *mut ::std::os::raw::c_void)>,
    pub data: *mut ::std::os::raw::c_void,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub irq: *mut Struct_avr_irq_t,
    pub r: Struct_Unnamed12,
    pub w: Struct_Unnamed13,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub param: *mut ::std::os::raw::c_void,
    pub c: avr_io_read_t,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed13 {
    pub param: *mut ::std::os::raw::c_void,
    pub c: avr_io_write_t,
}
impl ::std::clone::Clone for Struct_Unnamed13 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub used: ::std::os::raw::c_int,
    pub io: [Struct_Unnamed15; 4usize],
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub param: *mut ::std::os::raw::c_void,
    pub c: *mut ::std::os::raw::c_void,
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_t = Struct_avr_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_kind_t {
    pub names: [*const ::std::os::raw::c_char; 4usize],
    pub make: ::std::option::Option<extern "C" fn() -> *mut avr_t>,
}
impl ::std::clone::Clone for Struct_avr_kind_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_kind_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_kind_t = Struct_avr_kind_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_symbol_t {
    pub addr: uint32_t,
    pub symbol: [::std::os::raw::c_char; 0usize],
}
impl ::std::clone::Clone for Struct_avr_symbol_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_symbol_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_symbol_t = Struct_avr_symbol_t;
pub type avr_logger_p =
    ::std::option::Option<unsafe extern "C" fn(avr: *mut Struct_avr_t,
                                               level: ::std::os::raw::c_int,
                                               format:
                                                   *const ::std::os::raw::c_char,
                                               ap: va_list::VaList)>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_avr_io_t {
    pub next: *mut Struct_avr_io_t,
    pub avr: *mut avr_t,
    pub kind: *const ::std::os::raw::c_char,
    pub irq_names: *mut *const ::std::os::raw::c_char,
    pub irq_ioctl_get: uint32_t,
    pub irq_count: ::std::os::raw::c_int,
    pub irq: *mut Struct_avr_irq_t,
    pub reset: ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut Struct_avr_io_t)>,
    pub ioctl: ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut Struct_avr_io_t,
                                                          ctl: uint32_t,
                                                          io_param:
                                                              *mut ::std::os::raw::c_void)
                                         -> ::std::os::raw::c_int>,
    pub dealloc: ::std::option::Option<unsafe extern "C" fn(io:
                                                                *mut Struct_avr_io_t)>,
}
impl ::std::clone::Clone for Struct_avr_io_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_avr_io_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type avr_io_t = Struct_avr_io_t;
#[link(name = "simavr")]
extern "C" {
    pub fn avr_alloc_irq(pool: *mut avr_irq_pool_t, base: uint32_t,
                         count: uint32_t,
                         names: *mut *const ::std::os::raw::c_char)
     -> *mut avr_irq_t;
    pub fn avr_free_irq(irq: *mut avr_irq_t, count: uint32_t);
    pub fn avr_init_irq(pool: *mut avr_irq_pool_t, irq: *mut avr_irq_t,
                        base: uint32_t, count: uint32_t,
                        names: *mut *const ::std::os::raw::c_char);
    pub fn avr_raise_irq(irq: *mut avr_irq_t, value: uint32_t);
    pub fn avr_connect_irq(src: *mut avr_irq_t, dst: *mut avr_irq_t);
    pub fn avr_unconnect_irq(src: *mut avr_irq_t, dst: *mut avr_irq_t);
    pub fn avr_irq_register_notify(irq: *mut avr_irq_t,
                                   notify: avr_irq_notify_t,
                                   param: *mut ::std::os::raw::c_void);
    pub fn avr_irq_unregister_notify(irq: *mut avr_irq_t,
                                     notify: avr_irq_notify_t,
                                     param: *mut ::std::os::raw::c_void);
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
    pub fn strtoimax(__nptr: *const ::std::os::raw::c_char,
                     __endptr: *mut *mut ::std::os::raw::c_char,
                     __base: ::std::os::raw::c_int) -> intmax_t;
    pub fn strtoumax(__nptr: *const ::std::os::raw::c_char,
                     __endptr: *mut *mut ::std::os::raw::c_char,
                     __base: ::std::os::raw::c_int) -> uintmax_t;
    pub fn wcstoimax(__nptr: *const __gwchar_t,
                     __endptr: *mut *mut __gwchar_t,
                     __base: ::std::os::raw::c_int) -> intmax_t;
    pub fn wcstoumax(__nptr: *const __gwchar_t,
                     __endptr: *mut *mut __gwchar_t,
                     __base: ::std::os::raw::c_int) -> uintmax_t;
    pub fn avr_register_vector(avr: *mut Struct_avr_t,
                               vector: *mut avr_int_vector_t);
    pub fn avr_raise_interrupt(avr: *mut Struct_avr_t,
                               vector: *mut avr_int_vector_t)
     -> ::std::os::raw::c_int;
    pub fn avr_has_pending_interrupts(avr: *mut Struct_avr_t)
     -> ::std::os::raw::c_int;
    pub fn avr_is_interrupt_pending(avr: *mut Struct_avr_t,
                                    vector: *mut avr_int_vector_t)
     -> ::std::os::raw::c_int;
    pub fn avr_clear_interrupt(avr: *mut Struct_avr_t,
                               vector: *mut avr_int_vector_t);
    pub fn avr_service_interrupts(avr: *mut Struct_avr_t);
    pub fn avr_interrupt_reti(avr: *mut Struct_avr_t);
    pub fn avr_clear_interrupt_if(avr: *mut Struct_avr_t,
                                  vector: *mut avr_int_vector_t, old: uint8_t)
     -> ::std::os::raw::c_int;
    pub fn avr_get_interrupt_irq(avr: *mut Struct_avr_t, v: uint8_t)
     -> *mut avr_irq_t;
    pub fn avr_interrupt_init(avr: *mut Struct_avr_t);
    pub fn avr_interrupt_reset(avr: *mut Struct_avr_t);
    pub fn avr_cycle_timer_register(avr: *mut Struct_avr_t,
                                    when: avr_cycle_count_t,
                                    timer: avr_cycle_timer_t,
                                    param: *mut ::std::os::raw::c_void);
    pub fn avr_cycle_timer_register_usec(avr: *mut Struct_avr_t,
                                         when: uint32_t,
                                         timer: avr_cycle_timer_t,
                                         param: *mut ::std::os::raw::c_void);
    pub fn avr_cycle_timer_cancel(avr: *mut Struct_avr_t,
                                  timer: avr_cycle_timer_t,
                                  param: *mut ::std::os::raw::c_void);
    pub fn avr_cycle_timer_status(avr: *mut Struct_avr_t,
                                  timer: avr_cycle_timer_t,
                                  param: *mut ::std::os::raw::c_void)
     -> avr_cycle_count_t;
    pub fn avr_cycle_timer_process(avr: *mut Struct_avr_t)
     -> avr_cycle_count_t;
    pub fn avr_cycle_timer_reset(avr: *mut Struct_avr_t);
    pub fn avr_make_mcu_by_name(name: *const ::std::os::raw::c_char)
     -> *mut avr_t;
    pub fn avr_init(avr: *mut avr_t) -> ::std::os::raw::c_int;
    pub fn avr_core_allocate(core: *const avr_t, coreLen: uint32_t)
     -> *mut avr_t;
    pub fn avr_reset(avr: *mut avr_t);
    pub fn avr_run(avr: *mut avr_t) -> CPUState;
    pub fn avr_terminate(avr: *mut avr_t);
    pub fn avr_set_command_register(avr: *mut avr_t, addr: avr_io_addr_t);
    pub fn avr_set_console_register(avr: *mut avr_t, addr: avr_io_addr_t);
    pub fn avr_loadcode(avr: *mut avr_t, code: *mut uint8_t, size: uint32_t,
                        address: avr_flashaddr_t);
    pub fn avr_core_watch_write(avr: *mut avr_t, addr: uint16_t, v: uint8_t);
    pub fn avr_core_watch_read(avr: *mut avr_t, addr: uint16_t) -> uint8_t;
    pub fn avr_sadly_crashed(avr: *mut avr_t, signal: uint8_t);
    pub fn avr_global_logger(avr: *mut Struct_avr_t,
                             level: ::std::os::raw::c_int,
                             format: *const ::std::os::raw::c_char, ...);
    pub fn avr_global_logger_set(logger: avr_logger_p);
    pub fn avr_global_logger_get() -> avr_logger_p;
    pub fn avr_callback_sleep_gdb(avr: *mut avr_t,
                                  howLong: avr_cycle_count_t);
    pub fn avr_callback_run_gdb(avr: *mut avr_t);
    pub fn avr_callback_sleep_raw(avr: *mut avr_t,
                                  howLong: avr_cycle_count_t);
    pub fn avr_callback_run_raw(avr: *mut avr_t);
    pub fn avr_pending_sleep_usec(avr: *mut avr_t, howLong: avr_cycle_count_t)
     -> uint32_t;
    pub fn avr_register_io(avr: *mut avr_t, io: *mut avr_io_t);
    pub fn avr_io_setirqs(io: *mut avr_io_t, ctl: uint32_t,
                          count: ::std::os::raw::c_int, irqs: *mut avr_irq_t)
     -> *mut avr_irq_t;
    pub fn avr_register_io_read(avr: *mut avr_t, addr: avr_io_addr_t,
                                read: avr_io_read_t,
                                param: *mut ::std::os::raw::c_void);
    pub fn avr_register_io_write(avr: *mut avr_t, addr: avr_io_addr_t,
                                 write: avr_io_write_t,
                                 param: *mut ::std::os::raw::c_void);
    pub fn avr_ioctl(avr: *mut avr_t, ctl: uint32_t,
                     io_param: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
    pub fn avr_io_getirq(avr: *const avr_t, ctl: uint32_t,
                         index: ::std::os::raw::c_int)
     -> *mut Struct_avr_irq_t;
    pub fn avr_iomem_getirq(avr: *const avr_t, addr: avr_io_addr_t,
                            name: *const ::std::os::raw::c_char,
                            index: ::std::os::raw::c_int) -> *mut avr_irq_t;
    pub fn avr_deallocate_ios(avr: *mut avr_t);
}
