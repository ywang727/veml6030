from invoke import task

@task
def test(c):
    """Run driver library unit tests (host)"""
    print("--- Running Driver Unit Tests ---")
    # 同步测试
    c.run("cargo test -p veml6030_driver --no-default-features")
    # 异步测试 (注意：不开启 defmt，以防宿主机链接失败)
    c.run("cargo test -p veml6030_driver --no-default-features --features async")

@task
def run(c):
    """Compile and flash app firmware to nRF52840"""
    print("--- Flashing and Running App Firmware ---")
    with c.cd("veml6030_app"):
        c.run("cargo run")

@task
def build_app(c):
    """Build app firmware """
    print("--- Building Firmware ---")
    with c.cd("veml6030_app"):
        c.run("cargo build")

@task
def build_lib(c):
    """Build driver library """
    print("--- Building Driver Library ---")
    # 默认开启所有特性进行编译检查
    c.run("cargo build -p veml6030_driver --all-features")

@task
def clippy(c):
    """Run code static check"""
    print("--- Running Clippy ---")
    # 1. 检查同步模式 (关闭 async)
    c.run("cargo clippy -p veml6030_driver --no-default-features --features defmt")
    # 2. 检查异步模式 (开启 async)
    c.run("cargo clippy -p veml6030_driver --no-default-features --features async,defmt")
    # 3. 检查 App
    with c.cd("veml6030_app"):
        c.run("cargo clippy")

@task
def clean(c):
    """Clean all build products"""
    print("--- Cleaning Workspace ---")
    c.run("cargo clean -p veml6030_app")


@task
def all(c):
    """运行测试、编译、烧录和静态检查 (完整流程)"""
    test(c)
    build_lib(c)
    build_app(c)
    clippy(c)
    # run(c) # 运行固件 (可选)
