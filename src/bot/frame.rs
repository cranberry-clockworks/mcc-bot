use crate::bot::respondents;

#[derive(Copy, Clone)]
pub enum Frame {
    Default,
    VacancyOpenDialog(VacancyOpenDialogState),
}

#[derive(Copy, Clone)]
pub enum VacancyOpenDialogState {
    TitleInput,
    DescriptionInput,
}

pub type Respondent = fn(&str, &mut Frame) -> Option<String>;

pub fn respondent_for_frame(frame: &Frame) -> Respondent {
    match frame {
        Frame::Default => respondents::default::response,
        Frame::VacancyOpenDialog(_) => todo!(),
    }
}
