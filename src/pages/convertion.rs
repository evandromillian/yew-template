use web_sys::{Event, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

pub use super::{convert_units, VolumeUnit, WeightUnit};

fn select_to_weight_unit(select: HtmlSelectElement) -> WeightUnit {
    match select.value().parse().unwrap_or(1) {
        2 => WeightUnit::Gram,
        3 => WeightUnit::Milligram,
        4 => WeightUnit::Microgram,
        5 => WeightUnit::Nanogram,
        6 => WeightUnit::Picogram,
        _ => WeightUnit::Kilogram,
    }
}

fn select_to_volume_unit(select: HtmlSelectElement) -> VolumeUnit {
    match select.value().parse().unwrap_or(1) {
        2 => VolumeUnit::Deciliter,
        3 => VolumeUnit::Milliliter,
        4 => VolumeUnit::Microliter,
        5 => VolumeUnit::Nanoliter,
        6 => VolumeUnit::Picoliter,
        _ => VolumeUnit::Liter,
    }
}

#[function_component(UnitConverter)]
pub fn unit_converter() -> Html {
    let input_value = use_state(|| 0.0);
    let converted_value = use_state(|| 0.0);
    let from_weight_unit = use_state(|| WeightUnit::Kilogram);
    let to_weight_unit = use_state(|| WeightUnit::Kilogram);
    let from_volume_unit = use_state(|| VolumeUnit::Liter);
    let to_volume_unit = use_state(|| VolumeUnit::Liter);

    let converted_value_clone = converted_value.clone();
    let from_weight_unit_clone = *from_weight_unit.clone();
    let from_volume_unit_clone = *from_volume_unit.clone();
    let convert_units_callback = {
        let input_value = *input_value.clone();
        let to_weight_unit = *to_weight_unit.clone();
        let to_volume_unit = *to_volume_unit.clone();

        Callback::from(move |_| {
            let new_value = convert_units(
                input_value,
                from_weight_unit_clone,
                from_volume_unit_clone,
                to_weight_unit,
                to_volume_unit,
            );
            converted_value_clone.set(new_value);
        })
    };

    let input_value_clone = input_value.clone();
    let set_input_callback = Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            input_value_clone.set(input.value().parse().unwrap_or(0.0));
        }
    });

    let from_weight_unit_clone = from_weight_unit.clone();
    let set_weight_input_callback = Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            from_weight_unit_clone.set(select_to_weight_unit(input));
        }
    });

    let to_weight_input_callback = Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            to_weight_unit.set(select_to_weight_unit(input));
        }
    });

    let from_volume_unit_clone = from_volume_unit.clone();
    let set_volume_input_callback = Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            from_volume_unit_clone.set(select_to_volume_unit(input));
        }
    });

    let to_volume_input_callback = Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            to_volume_unit.set(select_to_volume_unit(input));
        }
    });

    html! {
        <div class="container mt-5">
            <h2 class="text-center">{"Unit Conversion App"}</h2>
            <div class="row">

                <div class="col-md-6">
                    <label>{ "Value:" }</label>

                    <input type="number" value={(*input_value).clone().to_string()} oninput={set_input_callback} class="form-control" id="value" placeholder="Enter value"/>

                    <label>{ "From Weight Unit:" }</label>
                    <select onchange={set_weight_input_callback} class="form-control" id="from-weight" >
                        <option value="1">{ "Kilogram" }</option>
                        <option value="2">{ "Gram" }</option>
                        <option value="3">{ "Milligram" }</option>
                        <option value="4">{ "Microgram" }</option>
                        <option value="5">{ "Nanogram" }</option>
                        <option value="6">{ "Picogram" }</option>
                    </select>

                    <label>{ "From Volume Unit:" }</label>
                    <select onchange={set_volume_input_callback} class="form-control" id="from-volume">
                        <option value="1">{ "Liter" }</option>
                        <option value="2">{ "Deciliter" }</option>
                        <option value="3">{ "Milliliter" }</option>
                        <option value="4">{ "Microliter" }</option>
                    </select>
                </div>

                <div class="col-md-6">
                    <label>{ "Converted Value:" }</label>
                    <input type="number" value={(*converted_value).clone().to_string()} class="form-control" id="converted-value" readonly=true />

                    <label>{ "To Weight Unit:" }</label>
                    <select onchange={to_weight_input_callback} class="form-control" id="to-weight">
                        <option value="1">{ "Kilogram" }</option>
                        <option value="2">{ "Gram" }</option>
                        <option value="3">{ "Milligram" }</option>
                        <option value="4">{ "Microgram" }</option>
                        <option value="5">{ "Nanogram" }</option>
                        <option value="6">{ "Picogram" }</option>
                    </select>

                    <label>{ "To Volume Unit:" }</label>
                    <select onchange={to_volume_input_callback} class="form-control" id="to-volume">
                        <option value="1">{ "Liter" }</option>
                        <option value="2">{ "Deciliter" }</option>
                        <option value="3">{ "Centiliter" }</option>
                        <option value="4">{ "Milliliter" }</option>
                        <option value="5">{ "Microliter" }</option>
                    </select>

                </div>
            </div>

            <div class="row mt-4">
                <div class="col-md-12 text-center">
                    <button onclick={convert_units_callback} class="btn btn-primary" id="convert-btn">{ "Convert" }</button>
                </div>
            </div>
        </div>
    }
}
