use super::asm::ASM;

#[derive(Debug)]
pub enum ConditionalJumpTo {
    IfEnd,
    ElifEnd,
    Else,
    Elif,
}

impl ASM {
    pub fn if_start(&mut self, jump_to: ConditionalJumpTo, if_num: usize) {
        // println!("if_start jump_to {:?}", jump_to);

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => format!(".if_end_{}", if_num),
            ConditionalJumpTo::ElifEnd => panic!("Cannot jump to elif end from if start"),
            ConditionalJumpTo::Elif => format!(".elif_{}_0", if_num),
            ConditionalJumpTo::Else => format!(".else_{}", if_num),
        };

        let stack_member = self.stack_pop().unwrap();

        let instructions = vec![
            // if label
            format!(".if_{}:", if_num),
            format!("cmp {stack_member}, 0"),
            format!(";; if the comparison value is false, jump to the next label altogether"),
            format!("je {}", jump_to_label),
        ];

        self.unlock_register_from_stack_value(&stack_member);

        self.extend_current_label(instructions);
    }

    pub fn if_end(&mut self, jump_to: ConditionalJumpTo, elif_len: usize, if_num: usize) {
        // println!("if_end jump_to {:?}", jump_to);

        let jump_to_label = match jump_to {
            // we simply jump to the very next label here as this means that there is a single if block without any elif
            // or else
            ConditionalJumpTo::IfEnd => format!(".if_end_{}", if_num),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_{}_end", if_num, elif_len - 1),
            ConditionalJumpTo::Elif => panic!("Cannot jump to elif start from if end"),
            ConditionalJumpTo::Else => format!(".else_end_{}", if_num),
        };

        // if we ever enter the if block, then that's it, we can jump straight to the end of the else or the elif block
        let instructions = vec![format!("jmp {}", jump_to_label), format!(".if_end_{}:", if_num)];

        self.extend_current_label(instructions);
    }

    /// The label names for all elifs will be of the same format, i.e. <elif label name>_<elif_number>
    /// so that we can easily jump around
    pub fn elif_start(&mut self, elif_number: usize, jump_to: ConditionalJumpTo, if_num: usize) {
        // self.change_current_label(format!(".{}_{}", label_name, elif_number));

        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => panic!("cannot jump to if from elif"),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_{}_end", if_num, elif_number),
            ConditionalJumpTo::Elif => format!(".elif_{}_{}", if_num, elif_number + 1),
            ConditionalJumpTo::Else => format!(".else_{}", if_num),
        };

        let stack_member = self.stack_pop().unwrap();

        let instructions = vec![
            // if label
            format!(".elif_{}_{}:", if_num, elif_number),
            format!("cmp {stack_member}, 0"),
            format!(";; if the comparison value is false, jump to the next label altogether"),
            format!("je {}", jump_to_label),
        ];

        self.unlock_register_from_stack_value(&stack_member);

        self.extend_current_label(instructions);
    }

    // we need jump_to in case there is not else
    pub fn elif_end(&mut self, elif_number: usize, jump_to: ConditionalJumpTo, if_num: usize) {
        let jump_to_label = match jump_to {
            ConditionalJumpTo::IfEnd => panic!("Cannot jump to if end from elif end"),
            ConditionalJumpTo::ElifEnd => format!(".elif_{}_{}_end", if_num, elif_number),
            ConditionalJumpTo::Elif => panic!("Cannot jump to elif start from elif end"),
            ConditionalJumpTo::Else => format!(".else_end_{}", if_num),
        };

        // if we ever enter the if block, then that's it, we can jump straight to the end of the else or the elif block
        let instructions = vec![
            format!("jmp {}", jump_to_label),
            format!(".elif_{}_{}_end:", if_num, elif_number),
        ];

        self.extend_current_label(instructions);
    }

    /// The label name for else will be unique
    pub fn else_start(&mut self, if_num: usize) {
        self.add_to_current_label(format!(".else_{}:", if_num));
    }

    pub fn else_end(&mut self, if_num: usize) {
        self.add_to_current_label(format!(".else_end_{}:", if_num));
    }
}
