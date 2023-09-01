use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq, Debug, Copy)]
pub enum Subjects {
    Custom,
    LanguageEnglish,
    LanguageItalian,
    LanguageFrench,
    Robotics,
    ProgrammingJavascript,
    ProgrammingSolana,
}

impl Default for Subjects {
    fn default() -> Self {
        Subjects::Custom
    }
}
