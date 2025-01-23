use cc_ui_kit::prelude::*;

use log::info;
use lucide_leptos::ArrowRightFromLine;
use thaw::*;

fn main() {
  console_error_panic_hook::set_once();
  #[cfg(debug_assertions)]
  console_log::init_with_level(log::Level::Debug).unwrap();
  #[cfg(not(debug_assertions))]
  console_log::init_with_level(log::Level::Info).unwrap();
  info!("starting app");
  leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
  let open = RwSignal::new(false);
  let open_f = move || open.set(true);
  let theme = RwSignal::new({
    let mut theme = Theme::light();
    theme.color.color_brand_background = "#17171a".to_string();
    theme.color.color_brand_background_hover = "#2c2c32".to_string();
    theme.color.color_brand_background_pressed = "#2c2c32".to_string();
    theme
  });

  view! {
    <ConfigProvider theme>
      <OverlayDrawer open position=DrawerPosition::Left>
        <DrawerHeader>
          <DrawerHeaderTitle>
            <DrawerHeaderTitleAction slot>
              <Button appearance=ButtonAppearance::Subtle on_click=move |_| open.set(false)>
                "x"
              </Button>
            </DrawerHeaderTitleAction>
            "Default Drawer"
          </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
          <p>"Drawer content"</p>
        </DrawerBody>
      </OverlayDrawer>
      <div class="ml-4 mr-4 mt-4">
        <Button appearance=ButtonAppearance::Primary on_click=move |_| open_f()>
          <ArrowRightFromLine color="white" size=24 stroke_width=2 />
          <p class="ml-2">"Hello there!"</p>
        </Button>
      </div>
      <Skeleton>
        <div class="p-4 space-y-2">
          <SkeletonItem />
          <SkeletonItem />
          <SkeletonItem />
        </div>
      </Skeleton>
    </ConfigProvider>
  }
}
