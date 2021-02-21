fn main() {
    windows::build!(
        windows::application_model::core::{
            CoreApplication,
            CoreApplicationView,
            IFrameworkViewSource,
            IFrameworkView,
        },
        windows::foundation::numerics::{Vector2, Vector3},
        windows::foundation::TimeSpan,
        windows::graphics::SizeInt32,
        windows::system::DispatcherQueueController,
        windows::ui::composition::{
            AnimationIterationBehavior,
            CompositionBatchTypes,
            CompositionBorderMode,
            CompositionColorBrush,
            CompositionGeometry,
            CompositionShape,
            CompositionSpriteShape,
            Compositor,
            ContainerVisual,
            SpriteVisual,
        },
        windows::ui::composition::desktop::DesktopWindowTarget,
        windows::ui::Colors,
        windows::ui::core::{
            CoreDispatcher,
            CoreWindow,
            CoreProcessEventsOption,
            WindowSizeChangedEventArgs,
            PointerEventArgs,
        },
        windows::win32::system_services::CreateDispatcherQueueController,
        windows::win32::winrt::{ICompositorDesktopInterop, RoInitialize},
    );
}
