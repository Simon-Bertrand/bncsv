#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::TempDir;
    use predicates::prelude::*;
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    use rand::Rng;

    fn bncsv() -> Command {
        Command::cargo_bin("bncsv").unwrap()
    }

    fn create_random_csv(csv_path: &PathBuf) -> (PathBuf) {
        if let Some(parent) = csv_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut wtr = File::create(&csv_path).unwrap();
        let mut rng = rand::thread_rng();
        let n_cols = 8;
        (0..1024).for_each(|_| {
            for (i, row) in (0..n_cols)
                .map(|_| {
                    rng.gen_range(-1000.0..1000.0)
                        .to_string()
                        .bytes()
                        .collect::<Vec<u8>>()
                })
                .enumerate()
            {
                wtr.write_all(&row).expect("Failed to write to file");
                if i != n_cols - 1 {
                    wtr.write(&[b',']).expect("Failed to write to file");
                }
                wtr.flush().unwrap();
            }
            wtr.write(&[b'\n']).expect("Failed to write to file");
            wtr.flush().unwrap();
        });
        (csv_path.to_owned())
    }

    fn run_cli_command(
        inputs: &Vec<String>,
        input_type: &str,
        output: Option<&PathBuf>,
        abs_pathbase: Option<&PathBuf>,
    ) -> Command {
        let mut bncsv_cmd = bncsv();
        let bncsv_args =
            bncsv_cmd.args(inputs.iter().map(|x| x.as_str()).chain(["-i", input_type]));
        if let Some(output_type) = output {
            bncsv_args.arg("-o").arg(output_type);
        }
        if let Some(abs_pb) = abs_pathbase {
            bncsv_args.arg("--abs-pathbase").arg(abs_pb);
        }
        bncsv_cmd
    }

    fn assert_file(path: &PathBuf) {
        assert!(path.exists(), "File {} does not exist", path.display());
        assert!(path.is_file(), "File {} is not a file", path.display());
        assert!(
            path.metadata().unwrap().len() > 0,
            "File {} is empty",
            path.display()
        );
    }

    #[test]
    fn test_cli_help() {
        bncsv()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage: bncsv"));
    }
    #[test]
    fn test_encode_decode() {
        let root = TempDir::new().unwrap();
        let mut csv_files = vec![];
        let fake_dir = root.join("fake");
        let out_dir = root.join("out");
        let gt_dir = root.join("gt");
        fs::create_dir_all(&fake_dir).unwrap();
        fs::create_dir_all(&out_dir).unwrap();
        fs::create_dir_all(&gt_dir).unwrap();
        let root_csv = create_random_csv(&fake_dir.join("root.csv"));
        csv_files.push(root_csv);
        for lvl1 in ["A", "B"] {
            for lvl2 in ["A", "B", "C"] {
                for i in 0..16 {
                    let p = fake_dir.join(lvl1).join(lvl2).join(format!("{}.csv", i));

                    match p.parent() {
                        Some(parent) if !parent.exists() => {
                            fs::create_dir_all(parent).unwrap();
                        }
                        _ => {}
                    }
                    csv_files.push(create_random_csv(&p.to_owned()));
                }
            }
        }

        run_cli_command(
            &vec![csv_files[0].to_path_buf().to_str().unwrap().to_string()],
            "csv",
            Some(&out_dir.join("root.bncsv")),
            None,
        )
        .assert()
        .success();
        run_cli_command(
            &vec![out_dir.join("root.bncsv").to_str().unwrap().to_string()],
            "bncsv",
            Some(&gt_dir.join("root.csv")),
            None,
        )
        .assert()
        .success();

        // Test level 1 'A' folders using positional args
        let mut temp = Vec::new();
        for lvl2 in ["A", "B", "C"] {
            for i in 0..16 {
                temp.push((&fake_dir).join("A").join(lvl2).join(format!("{}.csv", i)));
            }
        }

        run_cli_command(
            &temp
                .iter()
                .map(|x| x.to_str().unwrap().to_owned())
                .collect(),
            "csv",
            Some(&out_dir.join("A")),
            Some(&fake_dir.join("A")),
        )
        .assert()
        .success();

        let fakedir_a = (&fake_dir).join("A");
        let fakedir_a_segments = fakedir_a.into_iter().collect::<Vec<&OsStr>>();
        run_cli_command(
            &temp
                .iter()
                .map(|x| {
                    out_dir
                        .join("A")
                        .join(
                            x.iter()
                                .enumerate()
                                .skip_while(|(count, p)| {
                                    if count >= &fakedir_a_segments.len() {
                                        return false;
                                    }
                                    *p == fakedir_a_segments[*count]
                                }) // skip everything before "foo"
                                .map(|s| s.1)
                                .collect::<PathBuf>(),
                        )
                        .with_extension("bncsv")
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect(),
            "bncsv",
            Some(&gt_dir),
            Some(&out_dir),
        )
        .assert()
        .success();

        // Test level 1 'B' folders using glob pattern arg
        run_cli_command(
            &vec![(&fake_dir).join("B/**/*.csv").to_str().unwrap().to_string()],
            "csv",
            Some(&out_dir.join("B")),
            Some(&fake_dir.join("B")),
        )
        .assert()
        .success();

        run_cli_command(
            &vec![(&out_dir)
                .join("B/**/*.bncsv")
                .to_str()
                .unwrap()
                .to_string()],
            "bncsv",
            Some(&&gt_dir.join("B")),
            Some(&out_dir.join("B")),
        )
        .assert()
        .success();

        csv_files.iter().for_each(move |p| {
            let gt_p = gt_dir.join(p.strip_prefix(&fake_dir).unwrap());
            assert_file(&gt_p);
            assert!(std::fs::read(p)
                .expect("Failed to read file")
                .into_iter()
                .zip(
                    std::fs::read(gt_p)
                        .expect("Failed to read gt file")
                        .into_iter()
                )
                .all(|(b1, b2)| b1 == b2));
        });
    }
}
