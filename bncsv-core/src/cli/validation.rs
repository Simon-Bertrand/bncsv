use crate::Cli;

#[cfg(feature = "multithreading")]
use super::multithread::TaskQuery;

impl Cli {
    #[cfg(feature = "multithreading")]
    pub(crate) fn validate_multithreaded_tasks_paths(
        &self,
        tasks: &[TaskQuery],
    ) -> Result<(), std::io::Error> {
        if tasks.iter().any(|x| x.output_path.exists()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Overwriting files is disabled. Those paths already exist : {:?}",
                    tasks
                        .iter()
                        .find(|x| x.output_path.exists())
                        .map(|x| &x.output_path)
                        .unwrap()
                ),
            ));
        }
        Ok(())
    }
}
