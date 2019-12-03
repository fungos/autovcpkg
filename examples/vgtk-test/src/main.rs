use vgtk::{ext::*, gtk, run, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};

#[derive(Clone, Default, Debug)]
struct Model {
    counter: usize,
}

#[derive(Clone, Debug)]
enum Message {
   Inc,
   Exit,
}

impl Component for Model {
   type Message = Message;
   type Properties = ();

   fn update(&mut self, message: Message) -> UpdateAction<Self> {
       match message {
           Message::Inc => {
               self.counter += 1;
               UpdateAction::Render
           }
           Message::Exit => {
               vgtk::quit();
               UpdateAction::None
           }
       }
   }

   fn view(&self) -> VNode<Model> {
       gtk! {
           <Application::new_unwrap(None, ApplicationFlags::empty())>
               <Window border_width=20 on destroy=|_| Message::Exit>
                   <HeaderBar title="inc!" show_close_button=true />
                   <Box spacing=10 halign=Align::Center>
                       <Label label=self.counter.to_string() />
                       <Button label="inc!" image="add" always_show_image=true
                               on clicked=|_| Message::Inc />
                   </Box>
               </Window>
           </Application>
       }
   }
}

fn main() {
   std::process::exit(run::<Model>());
}
