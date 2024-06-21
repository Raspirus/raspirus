use leptos::*;

#[component]
pub fn VirusCard(
    title: String,
    text: String,
) -> impl IntoView {

    view! {
        <div className="flex mb-4 items-center shadow-md p-2 bg-white rounded-xl">
        <p className="whitespace-nowrap text-grey-darkest w-1/5 overflow-hidden">{title}</p>
        <div className="inline-block min-h-[1em] w-0.5 self-stretch bg-maingreen opacity-100 mx-2"></div>
        <p className="w-full text-grey-darkest overflow-x-auto">{text}</p>
    </div>
    }
}