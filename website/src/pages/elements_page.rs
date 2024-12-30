use leptos::{prelude::*, text_prop::TextProp};
use leptos_bulma::elements::{BAButton, BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicBlock, BasicBox, BasicTable, BasicTitle, RustCodeExample};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ElementsPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, elements) />

        <BTitle is=3>{t!(i18n, elements)}</BTitle>

        <BSection>
            <BTitle is=4>"Block"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_block" />

            <BBlock>"See it in action:"</BBlock>

            <BasicBlock />

            <GoToDocsRs path="elements/fn.BBlock" />
        </BSection>

        <BSection>
            <BTitle is=4>"Box"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_box" />

            <BBlock>"See it in action:"</BBlock>

            <BasicBox />

            <GoToDocsRs path="elements/fn.BBox" />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, button)}</BTitle>

            <BAButton is_fullwidth=true href="/elements/button">
                "Go to button page"
            </BAButton>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, icon)}</BTitle>

            <BAButton is_fullwidth=true href="/elements/icon">
                "Go to icon page"
            </BAButton>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, notification)}</BTitle>

            <BAButton is_fullwidth=true href="/elements/notification">
                "Go to notification page"
            </BAButton>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, progress)}</BTitle>

            <BAButton is_fullwidth=true href="/elements/progress">
                "Go to progress page"
            </BAButton>
        </BSection>

        <BSection>
            <BTitle is=4>"Table"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_table" />

            <BBlock>"See it in action:"</BBlock>

            <BasicTable />

            <GoToDocsRs path="elements/fn.BTable" />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, tag)}</BTitle>

            <BAButton is_fullwidth=true href="/elements/tag">
                "Go to tag page"
            </BAButton>
        </BSection>

        <BSection>
            <BTitle is=4>"Title"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_title" />

            <BBlock>"See it in action:"</BBlock>

            <BasicTitle />

            <GoToDocsRs path="elements/fn.BTitle" />
        </BSection>

        <GoToBulmaIo path="elements" />
    }
}
