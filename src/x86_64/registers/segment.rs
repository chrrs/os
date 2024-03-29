use gdt::SegmentSelector;

pub struct CodeSegment;

impl CodeSegment {
    pub fn write(selector: SegmentSelector) {
        unsafe {
            asm!("pushq $0; \
              leaq 1f(%rip), %rax; \
              pushq %rax; \
              lretq; \
              1:" :: "ri" (u64::from(selector.0)) : "rax" "memory")
        }
    }

    pub fn read() -> SegmentSelector {
        let out: u16;
        unsafe { asm!("mov $0, cs" : "=r" (out) ::: "intel") };
        SegmentSelector(out)
    }
}

pub struct DataSegment;

impl DataSegment {
    pub fn write(selector: SegmentSelector) {
        unsafe {
            asm!("mov ds, $0" :: "r" (u64::from(selector.0)) : "rax" "memory" : "intel")
        }
    }

    pub fn read() -> SegmentSelector {
        let out: u16;
        unsafe { asm!("mov $0, ds" : "=r" (out) ::: "intel") };
        SegmentSelector(out)
    }
}