use pb_jelly_gen::GenProtos;

fn main() -> std::io::Result<()> {
    GenProtos::builder()
        .out_path("../proto_types")
        .src_path("../proto_definition/")
        .cleanup_out_path(false)
        .gen_protos();

    Ok(())
}