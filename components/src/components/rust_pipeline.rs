//! Rust Pipeline view component - Wizard-driven 3-column layout
//! Shows the compilation pipeline: Rust -> WASM -> COR24 Assembly -> Machine Code -> Execution

use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Pre-built example for the Rust pipeline demo
#[derive(Clone, PartialEq)]
pub struct RustExample {
    pub name: String,
    pub description: String,
    pub rust_source: String,
    pub wasm_hex: String,
    pub wasm_size: usize,
    pub wasm_disassembly: String,
    pub cor24_assembly: String,
    pub machine_code_hex: String,
    pub machine_code_size: usize,
    pub listing: String,
}

/// CPU state for display in the Rust pipeline execution panel
#[derive(Clone, PartialEq, Default)]
pub struct RustCpuState {
    pub registers: [u32; 8],
    pub pc: u32,
    pub condition_flag: bool,
    pub is_halted: bool,
    pub led_value: u8,
    pub cycle_count: u32,
    pub memory_snapshot: Vec<u8>,
    pub current_instruction: String,
    pub assembled_lines: Vec<String>,
}

/// Wizard steps for progressive disclosure
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WizardStep {
    Source,    // Initial state - only Source visible
    Compile,   // WASM binary revealed
    Disasm,    // WASM disassembly revealed
    Translate, // COR24 assembly revealed
    Assemble,  // Debugger revealed
}

impl WizardStep {
    fn label(&self) -> &'static str {
        match self {
            WizardStep::Source => "Source",
            WizardStep::Compile => "Compile",
            WizardStep::Disasm => "Disasm",
            WizardStep::Translate => "Translate",
            WizardStep::Assemble => "Assemble",
        }
    }

    fn next(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Source => Some(WizardStep::Compile),
            WizardStep::Compile => Some(WizardStep::Disasm),
            WizardStep::Disasm => Some(WizardStep::Translate),
            WizardStep::Translate => Some(WizardStep::Assemble),
            WizardStep::Assemble => None,
        }
    }

    fn action_label(&self) -> &'static str {
        match self {
            WizardStep::Source => "Compile",
            WizardStep::Compile => "Disassemble",
            WizardStep::Disasm => "Translate",
            WizardStep::Translate => "Assemble",
            WizardStep::Assemble => "",
        }
    }

    fn cell_id(&self) -> &'static str {
        match self {
            WizardStep::Source => "cell-source",
            WizardStep::Compile => "cell-wasm",
            WizardStep::Disasm => "cell-disasm",
            WizardStep::Translate => "cell-cor24",
            WizardStep::Assemble => "cell-debug",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RustPipelineProps {
    pub examples: Vec<RustExample>,
    pub on_load: Callback<RustExample>,
    pub on_step: Callback<()>,
    pub on_run: Callback<()>,
    pub on_reset: Callback<()>,
    #[prop_or_default]
    pub on_unload: Callback<()>,
    pub cpu_state: RustCpuState,
    pub is_loaded: bool,
    pub is_running: bool,
    // Modal callbacks - passed from app.rs
    #[prop_or_default]
    pub on_tutorial_open: Callback<()>,
    #[prop_or_default]
    pub on_examples_open: Callback<()>,
    #[prop_or_default]
    pub on_isa_ref_open: Callback<()>,
    #[prop_or_default]
    pub on_help_open: Callback<()>,
}

#[function_component(RustPipeline)]
pub fn rust_pipeline(props: &RustPipelineProps) -> Html {
    // Wizard step state
    let current_step = use_state(|| WizardStep::Source);

    // Load dialog state
    let load_dialog_open = use_state(|| false);

    // Selected example state
    let selected_example = use_state(|| {
        props.examples.first().cloned()
    });

    // Track PC for auto-scroll
    let last_pc = use_state(|| 0u32);

    // Auto-scroll current assembly line into view when PC changes
    // Only scrolls within the listing container, not the whole page
    {
        let pc = props.cpu_state.pc;
        let last_pc = last_pc.clone();
        use_effect_with(pc, move |&current_pc| {
            if current_pc != *last_pc {
                last_pc.set(current_pc);
                // Scroll within the listing container only
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(container) = document.get_element_by_id("asm-listing-scroll") {
                            if let Some(element) = document.query_selector(".asm-line.current-line").ok().flatten() {
                                let element_html: &web_sys::HtmlElement = element.unchecked_ref();
                                let container_html: &web_sys::HtmlElement = container.unchecked_ref();
                                let element_top = element_html.offset_top();
                                let container_height = container_html.client_height();
                                let container_scroll = container.scroll_top();

                                // Only scroll if element is outside visible area
                                let element_bottom = element_top + 20; // approx line height
                                let visible_top = container_scroll;
                                let visible_bottom = container_scroll + container_height;

                                if element_top < visible_top || element_bottom > visible_bottom {
                                    // Center the element in the container
                                    let target_scroll = element_top - (container_height / 2);
                                    container.set_scroll_top(target_scroll.max(0));
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    // Example selector callback
    let on_example_select = {
        let selected_example = selected_example.clone();
        let examples = props.examples.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
            if let Some(select) = target {
                let idx = select.selected_index() as usize;
                if let Some(example) = examples.get(idx) {
                    selected_example.set(Some(example.clone()));
                }
            }
        })
    };

    // Open load dialog
    let on_load_dialog_open = {
        let load_dialog_open = load_dialog_open.clone();
        Callback::from(move |_| {
            load_dialog_open.set(true);
        })
    };

    // Close load dialog
    let on_load_dialog_close = {
        let load_dialog_open = load_dialog_open.clone();
        Callback::from(move |_| {
            load_dialog_open.set(false);
        })
    };

    // Load example callback - resets to Source step and closes dialog
    let on_load_click = {
        let on_load = props.on_load.clone();
        let selected = selected_example.clone();
        let current_step = current_step.clone();
        let load_dialog_open = load_dialog_open.clone();
        Callback::from(move |_| {
            if let Some(example) = &*selected {
                on_load.emit(example.clone());
                current_step.set(WizardStep::Source);
                load_dialog_open.set(false);
            }
        })
    };

    // Advance to next wizard step with scroll
    let advance_step = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            let current = *current_step;
            if let Some(next) = current.next() {
                current_step.set(next);
                // Determine which cell to scroll to:
                // - For Assemble step: scroll to Execution cell (the new cell)
                // - For other steps: scroll to keep previous cell (N-1) at top
                let scroll_to_cell = if next == WizardStep::Assemble {
                    next.cell_id().to_string() // Scroll to Execution
                } else {
                    current.cell_id().to_string() // Scroll to previous cell (N-1)
                };

                gloo::timers::callback::Timeout::new(100, move || {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(container) = document.get_element_by_id("notebook-scroll") {
                                if let Some(element) = document.get_element_by_id(&scroll_to_cell) {
                                    let element_html: &web_sys::HtmlElement = element.unchecked_ref();
                                    container.set_scroll_top(element_html.offset_top());
                                }
                            }
                        }
                    }
                }).forget();
            }
        })
    };

    // Click on a wizard step to jump to it
    let on_step_click = {
        let current_step = current_step.clone();
        move |step: WizardStep| {
            let current_step = current_step.clone();
            Callback::from(move |_| {
                if step <= *current_step {
                    // Scroll to the cell - align at top
                    let cell_id = step.cell_id().to_string();
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(container) = document.get_element_by_id("notebook-scroll") {
                                if let Some(element) = document.get_element_by_id(&cell_id) {
                                    let element_html: &web_sys::HtmlElement = element.unchecked_ref();
                                    container.set_scroll_top(element_html.offset_top());
                                }
                            }
                        }
                    }
                }
            })
        }
    };

    // Reset wizard to Source step (always enabled) - also unloads the example
    let on_wizard_reset = {
        let current_step = current_step.clone();
        let on_unload = props.on_unload.clone();
        let selected_example = selected_example.clone();
        Callback::from(move |_| {
            current_step.set(WizardStep::Source);
            selected_example.set(None);
            on_unload.emit(());
        })
    };

    // Execution callbacks
    let on_step_click_exec = {
        let on_step = props.on_step.clone();
        Callback::from(move |_| on_step.emit(()))
    };

    let on_run_click = {
        let on_run = props.on_run.clone();
        Callback::from(move |_| on_run.emit(()))
    };

    let on_reset_click = {
        let on_reset = props.on_reset.clone();
        Callback::from(move |_| on_reset.emit(()))
    };

    let state = &props.cpu_state;

    // All wizard steps for rendering
    let all_steps = [
        WizardStep::Source,
        WizardStep::Compile,
        WizardStep::Disasm,
        WizardStep::Translate,
        WizardStep::Assemble,
    ];

    html! {
        <div class="rust-wizard-layout">
            // Column 1: Sidebar with buttons and peripherals
            <div class="wizard-sidebar">
                <div class="wizard-buttons">
                    <button onclick={
                        let cb = props.on_tutorial_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Tutorial"}</button>
                    <button onclick={
                        let cb = props.on_examples_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Examples"}</button>
                    <button onclick={
                        let cb = props.on_isa_ref_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"ISA Ref"}</button>
                    <button onclick={
                        let cb = props.on_help_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Help"}</button>
                </div>

                // LEDs and peripherals at bottom
                <div class="wizard-peripherals">
                    <div class="peripheral-section">
                        <div class="peripheral-label">{"LEDs"}</div>
                        <div class="led-row-vertical">
                            {for (0..8).rev().map(|i| {
                                let led_on = (state.led_value >> i) & 1 == 1;
                                let class = if led_on { "led led-on" } else { "led led-off" };
                                html! {
                                    <div class={class}>{i}</div>
                                }
                            })}
                        </div>
                        <div class="led-hex">{format!("0x{:02X}", state.led_value)}</div>
                    </div>
                    <div class="peripheral-section">
                        <div class="peripheral-label">{"I2C"}</div>
                        <div class="peripheral-status">{"(not connected)"}</div>
                    </div>
                    <div class="peripheral-section">
                        <div class="peripheral-label">{"RTC"}</div>
                        <div class="peripheral-status">{"--"}</div>
                    </div>
                </div>
            </div>

            // Column 2: Wizard steps
            <div class="wizard-steps">
                <div class="wizard-header-space"></div>

                {for all_steps.iter().map(|&step| {
                    let is_completed = step < *current_step;
                    let is_active = step == *current_step;
                    let is_disabled = step > *current_step;

                    let class = classes!(
                        "wizard-step",
                        is_active.then_some("active"),
                        is_completed.then_some("completed"),
                        is_disabled.then_some("disabled"),
                    );

                    let indicator = if is_completed { "✓" } else { "○" };

                    html! {
                        <div
                            class={class}
                            onclick={if !is_disabled { Some(on_step_click(step)) } else { None }}
                        >
                            <span class="step-indicator">{indicator}</span>
                            <span class="step-label">{step.label()}</span>
                        </div>
                    }
                })}

                // Action button - Load when not loaded, then Compile/Disassemble/etc.
                if !props.is_loaded {
                    <button class="wizard-action-btn" onclick={on_load_dialog_open.clone()} disabled={props.is_running}>
                        {"Load"}
                    </button>
                } else if *current_step != WizardStep::Assemble {
                    <button class="wizard-action-btn" onclick={advance_step.clone()}>
                        {(*current_step).action_label()}
                    </button>
                }

                // Reset button - always enabled, red color
                <button class="wizard-reset-btn" onclick={on_wizard_reset}>
                    {"Reset"}
                </button>
            </div>

            // Column 3: Notebook cells
            <div class="notebook-cells" id="notebook-scroll">
                if let Some(example) = &*selected_example {
                    // Cell 1: Rust Source (always visible)
                    <div class="notebook-cell" id="cell-source">
                        <div class="cell-header">{"Rust Source"}</div>
                        <div class="cell-content">
                            <pre class="code-block rust-code">{&example.rust_source}</pre>
                        </div>
                    </div>

                    // Cell 2: WASM Binary (visible after Compile step)
                    if *current_step >= WizardStep::Compile {
                        <div class="notebook-cell" id="cell-wasm">
                            <div class="cell-header">
                                <span>{"WASM Binary"}</span>
                                <span class="cell-badge">{format!("{} bytes", example.wasm_size)}</span>
                            </div>
                            <div class="cell-content">
                                <pre class="code-block hex-dump">{&example.wasm_hex}</pre>
                            </div>
                        </div>
                    }

                    // Cell 3: WASM Disassembly (visible after Disasm step)
                    if *current_step >= WizardStep::Disasm {
                        <div class="notebook-cell" id="cell-disasm">
                            <div class="cell-header">{"WASM Disassembly"}</div>
                            <div class="cell-content">
                                <pre class="code-block wasm-disasm">{&example.wasm_disassembly}</pre>
                            </div>
                        </div>
                    }

                    // Cell 4: COR24 Assembly (visible after Translate step)
                    if *current_step >= WizardStep::Translate {
                        <div class="notebook-cell" id="cell-cor24">
                            <div class="cell-header">{"COR24 Assembly"}</div>
                            <div class="cell-content">
                                <pre class="code-block asm-code">{&example.cor24_assembly}</pre>
                            </div>
                        </div>
                    }

                    // Cell 5: Debug Panel (visible after Assemble step)
                    if *current_step >= WizardStep::Assemble {
                        <div class="notebook-cell notebook-cell-debug" id="cell-debug">
                            <div class="cell-header">{"Execution"}</div>
                            <div class="debug-panel">
                                // Debug controls
                                <div class="debug-controls">
                                    <button class="step-btn" onclick={on_step_click_exec.clone()}
                                        disabled={!props.is_loaded || state.is_halted || props.is_running}>
                                        {"Step"}
                                    </button>
                                    <button class="run-btn" onclick={on_run_click.clone()}
                                        disabled={!props.is_loaded || state.is_halted || props.is_running}>
                                        {if props.is_running { "Running..." } else { "Run" }}
                                    </button>
                                    <button class="reset-btn" onclick={on_reset_click.clone()}
                                        disabled={!props.is_loaded || props.is_running}>
                                        {"Reset"}
                                    </button>
                                    if state.is_halted {
                                        <span class="status-halted">{"HALTED"}</span>
                                    }
                                </div>

                                // Two-column debug content
                                <div class="debug-content">
                                    // Left: Assembly listing
                                    <div class="debug-left">
                                        <h4>{"Assembly"}</h4>
                                        if props.is_loaded && !state.assembled_lines.is_empty() {
                                            <div class="listing-scroll" id="asm-listing-scroll">
                                                {for state.assembled_lines.iter().map(|line| {
                                                    let is_current = if line.len() > 4 && line.chars().nth(4) == Some(':') {
                                                        if let Ok(addr) = u32::from_str_radix(&line[0..4], 16) {
                                                            addr == state.pc
                                                        } else {
                                                            false
                                                        }
                                                    } else {
                                                        false
                                                    };
                                                    let class = if is_current { "asm-line current-line" } else { "asm-line" };
                                                    html! {
                                                        <div class={class}>{line}</div>
                                                    }
                                                })}
                                            </div>
                                        }
                                    </div>

                                    // Right: Registers and state
                                    <div class="debug-right">
                                        <h4>{"Registers"}</h4>
                                        <div class="register-grid-compact">
                                            {for (0..8).map(|i| {
                                                let name = match i {
                                                    0 => "r0",
                                                    1 => "r1",
                                                    2 => "r2",
                                                    3 => "fp",
                                                    4 => "sp",
                                                    5 => "z",
                                                    6 => "iv",
                                                    7 => "ir",
                                                    _ => "??",
                                                };
                                                let val = state.registers[i];
                                                html! {
                                                    <div class="register-row-compact">
                                                        <span class="reg-name">{name}</span>
                                                        <span class="reg-value">{format!("0x{:06X}", val)}</span>
                                                    </div>
                                                }
                                            })}
                                            <div class="register-row-compact">
                                                <span class="reg-name">{"PC"}</span>
                                                <span class="reg-value">{format!("0x{:06X}", state.pc)}</span>
                                            </div>
                                            <div class="register-row-compact">
                                                <span class="reg-name">{"C"}</span>
                                                <span class="reg-value">{if state.condition_flag { "1" } else { "0" }}</span>
                                            </div>
                                        </div>

                                        <div class="cycle-info">
                                            <span>{"Cycles: "}{state.cycle_count}</span>
                                        </div>

                                        // Memory viewer (first 64 bytes of stack area)
                                        if props.is_loaded && !state.memory_snapshot.is_empty() {
                                            <div class="memory-section">
                                                <h4>{"Memory (Stack)"}</h4>
                                                <pre class="memory-dump-compact">{
                                                    format_memory_dump(&state.memory_snapshot, 0xFFFFC0)
                                                }</pre>
                                            </div>
                                        }
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    // No example loaded - show placeholder
                    <div class="notebook-placeholder">
                        <p>{"Click "}<strong>{"Load Example"}</strong>{" to get started."}</p>
                    </div>
                }

                // Future: Server-side compilation notice
                <div class="pipeline-note">
                    <em>{"Note: Examples are pre-built. Server-side compilation coming soon."}</em>
                </div>
            </div>

            // Load Example Dialog
            if *load_dialog_open {
                <div class="wizard-dialog-overlay" onclick={on_load_dialog_close.clone()}>
                    <div class="wizard-dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <div class="wizard-dialog-header">
                            <h3>{"Load Example"}</h3>
                            <button class="wizard-dialog-close" onclick={on_load_dialog_close.clone()}>{"×"}</button>
                        </div>
                        <div class="wizard-dialog-body">
                            <label>{"Select an example:"}</label>
                            <select class="wizard-dialog-select" onchange={on_example_select} disabled={props.is_running}>
                                {for props.examples.iter().map(|ex| {
                                    html! {
                                        <option value={ex.name.clone()}>{&ex.name}{" - "}{&ex.description}</option>
                                    }
                                })}
                            </select>
                        </div>
                        <div class="wizard-dialog-footer">
                            <button class="wizard-dialog-cancel" onclick={on_load_dialog_close}>{"Cancel"}</button>
                            <button class="wizard-dialog-load" onclick={on_load_click} disabled={props.is_running}>{"Load"}</button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}

/// Format memory as hex dump with addresses
fn format_memory_dump(data: &[u8], base_addr: u32) -> String {
    let mut output = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        let addr = base_addr + (i * 16) as u32;
        output.push_str(&format!("{:06X}: ", addr));
        for (j, byte) in chunk.iter().enumerate() {
            output.push_str(&format!("{:02X} ", byte));
            if j == 7 {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    output
}
