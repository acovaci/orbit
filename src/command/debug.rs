pub fn run(args: Vec<String>) {
    crate::python::interface::dbt_run("debug", args);
}
