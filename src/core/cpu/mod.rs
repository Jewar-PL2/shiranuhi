mod cop0;

#[derive(Clone, Copy)]
struct DelaySlot {
    register: usize,
    value: u32
}

pub struct Cpu {
    regs: [u32; 32],
    program_counter: u32,
    program_counter_predictor: u32,

    delay_slots: [Option<DelaySlot>; 2]
}

impl Cpu {
    pub fn new() -> Self {
        let mut regs = [0xDEADBEEF; 32];
        regs[0] = 0;
        
        Self {
            regs,
            program_counter: 0xBFC00000,
            program_counter_predictor: 0xBFC00004,
            delay_slots: [None; 2]
        }
    }

    // NOTE: delay slots mechanics are temporarily copied from https://github.com/JaCzekanski/Avocado
    // It will be replaced in the future, perhaps I don't even need two slots

    fn set_reg(&mut self, register: usize, value: u32) {
        if register == 0 {
            return;
        }
        self.regs[register] = value;

        if let Some(slot) = self.delay_slots[0] {
            if slot.register == register {
                self.delay_slots[0] = None;
            }
        }
    }

    fn load_delay_slot(&mut self, register: usize, value: u32) {
        if register == 0 {
            return
        }

        if let Some(slot) = self.delay_slots[0] {
            if register == slot.register {
                self.delay_slots[0] = None;
            }
        }

        self.delay_slots[1] = Some(DelaySlot { register, value })
    }

    fn move_delay_slots(&mut self) {
        if let Some(slot0) = self.delay_slots[0] {
            self.regs[slot0.register] = slot0.value;
        }

        self.delay_slots[0] = self.delay_slots[1].take();
    }
}