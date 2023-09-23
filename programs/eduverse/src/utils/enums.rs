use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Eq, PartialEq, Debug, Copy)]
pub enum Subjects {
    #[default]
    Custom,
    LanguageEnglish,
    LanguageItalian,
    LanguageFrench,
    Robotics,
    ProgrammingJavascript,
    ProgrammingSolana,
}
