pub struct DcFrame;


impl Component for App {
    type Message = ();
    type Properties = ();
    // 
    fn create(_ctx: &Context<Self>)-> Self{
        Self
    }
    //
    fn view(&self,_ctx:&Context<Self>)-> Html{

        html! {
            <>

            </>
        }
    }
}
