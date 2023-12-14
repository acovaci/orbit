pub fn run(args: Vec<String>) {
    crate::python::interface::dbt_run("run-operation", args);
}
