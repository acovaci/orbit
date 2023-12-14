/*

class DebugTask(BaseTask):
    def __init__(self, args, config) -> None:
        super().__init__(args, config)
        self.profiles_dir = args.PROFILES_DIR
        self.profile_path = os.path.join(self.profiles_dir, "profiles.yml")
        try:
            self.project_dir = get_nearest_project_dir(self.args.project_dir)
        except dbt.exceptions.Exception:
            # we probably couldn't find a project directory. Set project dir
            # to whatever was given, or default to the current directory.
            if args.project_dir:
                self.project_dir = args.project_dir
            else:
                self.project_dir = Path.cwd()
        self.project_path = os.path.join(self.project_dir, "dbt_project.yml")
        self.cli_vars: Dict[str, Any] = args.vars

        # set by _load_*
        self.profile: Optional[Profile] = None
        self.raw_profile_data: Optional[Dict[str, Any]] = None
        self.profile_name: Optional[str] = None
        self.project: Optional[Project] = None

*/

pub fn run(args: Vec<String>) {
    crate::python::interface::dbt_run("debug", args);
}
