use(asset) "icons/plus-circle" as PlusIcon

// use(asset) is an arcen style import of a renderable asset like an image or font
// they will usually be imported as an Image component or Font component
// you can check by hovering over it

# Counter component
fn Counter() -> Component {
    let count, set_count = use_state(0 as i64)

    @arcen PlusIcon[on_click=() => set_count(count+1) h=Rem(2)]
}
