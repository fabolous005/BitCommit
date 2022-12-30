use bollard::{
    Docker,
    image::{
        BuildImageOptions,
    },
    container::{
        Config,
    }
};


pub static DOCKER: Docker = Docker::connect_with_unix_defaults().unwrap();


pub async fn pull() -> Result<(), bollard::errors::Error>
{
    DOCKER.build_image(
            BuildImageOptions {
                pull: true,
                ..Default::default()
            },
            None,
            None,
        );
    Ok(())
}
