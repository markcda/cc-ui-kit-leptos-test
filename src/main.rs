use cc_ui_kit::prelude::*;
use leptos_router::components::Router;
use log::info;
use lucide_leptos::{ArrowRightFromLine, File};

fn main() {
  setup_app(
    log::Level::Info,
    Box::new(move || view! { <App /> }.into_any()),
  )
}

#[component]
fn App() -> impl IntoView {
  let open = RwSignal::new(false);
  let open_f = move || open.set(true);
  let value = RwSignal::new(String::new());
  let fvalue = RwSignal::new(0.0);
  let options = Memo::<Vec<_>>::new(move |_| {
    let prefix = value
      .get()
      .split_once('@')
      .map_or(value.get(), |v| v.0.to_string());
    vec!["@gmail.com", "@163.com"]
      .into_iter()
      .map(|suffix| (format!("{prefix}{suffix}"), format!("{prefix}{suffix}")))
      .collect()
  });
  let checked = RwSignal::new(false);
  let color_value = RwSignal::new(Color::from(palette::Srgb::new(0.0, 0.0, 0.0)));
  let selected_options = RwSignal::new(None::<String>);
  let open_di = RwSignal::new(false);

  view! {
    // <Router>
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
      <Button class="mr-4" appearance=ButtonAppearance::Primary on_click=move |_| open_f()>
        <ArrowRightFromLine size=24 stroke_width=2 />
        <p class="ml-2">"Hello there!"</p>
      </Button>
      <a href="/somefile.docx">
        <Button appearance=ButtonAppearance::Secondary>
          <File size=24 stroke_width=2 />
          <p class="ml-2">"Download this"</p>
        </Button>
      </a>
    </div>
    <button class="bg-pink-500 dark:bg-blue-500">"Hello!"</button>
    <Skeleton>
      <div class="p-4 space-y-2">
        <SkeletonItem />
        <SkeletonItem />
        <SkeletonItem />
      </div>
    </Skeleton>
    <Accordion>
      <AccordionItem value="leptos">
        <AccordionHeader slot>"Leptos"</AccordionHeader>
        "Build fast web applications with Rust."
      </AccordionItem>
      <AccordionItem value="thaw">
        <AccordionHeader slot>"Thaw"</AccordionHeader>
        "An easy to use leptos component library"
      </AccordionItem>
    </Accordion>
    <Anchor>
      <AnchorLink title="Web API" href="#web">
        <AnchorLink title="DOM" href="#dom" />
        <AnchorLink title="SVG" href="#svg" />
        <AnchorLink title="File API" href="#file" />
      </AnchorLink>
      <AnchorLink title="Rust" href="#rust" />
      <AnchorLink title="Anchor Props" href="#anchor-props" />
      <AnchorLink title="AnchorLink Props" href="#anchorlink-props" />
    </Anchor>
    <AutoComplete value placeholder="Email">
      <For each=move || options.get() key=|option| option.0.clone() let:option>
        <AutoCompleteOption value=option.0>{option.1}</AutoCompleteOption>
      </For>
    </AutoComplete>
    <Avatar />
    <BackTop />
    <Flex>
      <Badge appearance=BadgeAppearance::Filled>"999+"</Badge>
      <Badge appearance=BadgeAppearance::Ghost>"999+"</Badge>
      <Badge appearance=BadgeAppearance::Outline>"999+"</Badge>
      <Badge appearance=BadgeAppearance::Tint>"999+"</Badge>
    </Flex>
    <Breadcrumb>
      <BreadcrumbItem>
        <BreadcrumbButton>"Leptos"</BreadcrumbButton>
      </BreadcrumbItem>
      <BreadcrumbDivider />
      <BreadcrumbItem>
        <BreadcrumbButton>"UI"</BreadcrumbButton>
      </BreadcrumbItem>
      <BreadcrumbDivider />
      <BreadcrumbItem>
        <BreadcrumbButton current=true>"Thaw"</BreadcrumbButton>
      </BreadcrumbItem>
    </Breadcrumb>
    <Space>
      <Button>"Secondary"</Button>
      <Button appearance=ButtonAppearance::Primary>"Primary"</Button>
    </Space>
    <Space>
      <Button appearance=ButtonAppearance::Subtle>"Subtle"</Button>
      <Button appearance=ButtonAppearance::Transparent>"Transparent"</Button>
    </Space>
    <Card>
      <CardHeader>
        <Body1>
          <b>"Header"</b>
          " 2022-02-22"
        </Body1>
        <CardHeaderDescription slot>
          <Caption1>"Description"</Caption1>
        </CardHeaderDescription>
        <CardHeaderAction slot>
          <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
        </CardHeaderAction>
      </CardHeader>
      <CardPreview>
        <img src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" style="width: 100%" />
      </CardPreview>
      <CardFooter>
        <Button>"Reply"</Button>
        <Button>"Share"</Button>
      </CardFooter>
    </Card>
    <Checkbox checked label="Click" />
    <Checkbox />
    <ColorPicker value=color_value />
    <Combobox selected_options placeholder="Select an animal">
      <ComboboxOption value="cat" text="Cat" disabled=true />
      <ComboboxOption value="dog" text="Dog" />
    </Combobox>
    <Button on_click=move |_| open_di.set(true)>"Open Dialog"</Button>
    <Dialog open=open_di>
      <DialogSurface>
        <DialogBody>
          <DialogTitle>"Dialog title"</DialogTitle>
          <DialogContent>"Dialog body"</DialogContent>
          <DialogActions>
            <Button appearance=ButtonAppearance::Primary>"Do Something"</Button>
          </DialogActions>
        </DialogBody>
      </DialogSurface>
    </Dialog>
    <Space vertical=true>
      <div style="padding: 30px 0; background-color: var(--colorNeutralBackground1);">
        <Divider />
      </div>
      <div style="padding: 30px 0; background-color: var(--colorNeutralBackground1);">
        <Divider>"Text"</Divider>
      </div>
    </Space>
    <Field label="Example field">
      <Input />
    </Field>
    <InfoLabel>
      <InfoLabelInfo slot>"This is example information for an InfoLabel. "</InfoLabelInfo>
      "Example label"
    </InfoLabel>
    <NavDrawer>
      <NavCategory value="area">
        <NavCategoryItem slot icon=icondata::AiAreaChartOutlined>
          "Area Chart"
        </NavCategoryItem>
        <NavSubItem value="target">"Target"</NavSubItem>
        <NavSubItem value="above">"Above"</NavSubItem>
        <NavSubItem value="below">"Below"</NavSubItem>
      </NavCategory>
      <NavCategory value="pie">
        <NavCategoryItem slot icon=icondata::AiPieChartOutlined>
          "Pie Chart"
        </NavCategoryItem>
        <NavSubItem value="pie-target">"Pie Target"</NavSubItem>
        <NavSubItem value="pin-above">"Pin Above"</NavSubItem>
        <NavSubItem value="pin-below">"Pin Below"</NavSubItem>
      </NavCategory>
      <NavItem
        icon=icondata::AiGithubOutlined
        value="github"
        href="https://github.com/markcda/cc-ui-kit"
        attr:target="_blank"
      >
        "CC UI Kit's GitHub"
      </NavItem>
      <NavItem icon=icondata::AiChromeOutlined value="chrome">
        "Chrome"
      </NavItem>
    </NavDrawer>
    <Space vertical=true>
      <Space>
        <Tag>"Medium"</Tag>
        <Tag size=TagSize::Small>"Small"</Tag>
        <Tag size=TagSize::ExtraSmall>"Extra small"</Tag>
      </Space>
      <Space>
        <Tag dismissible=true>"Medium"</Tag>
        <Tag dismissible=true size=TagSize::Small>
          "Small"
        </Tag>
        <Tag dismissible=true size=TagSize::ExtraSmall>
          "Extra small"
        </Tag>
      </Space>
    </Space>
    <Spinner size=SpinnerSize::ExtraTiny label="Extra Tiny Spinner" />
    <Switch checked />
    <Slider value=fvalue />
  }
}
