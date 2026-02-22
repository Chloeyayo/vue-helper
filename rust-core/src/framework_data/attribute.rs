use super::AttrInfo;
use std::collections::HashMap;

macro_rules! attr {
    ($type:expr, $desc:expr) => {
        AttrInfo { attr_type: $type.to_string(), description: $desc.to_string(), values: vec![] }
    };
    ($type:expr, $desc:expr, $($val:expr),*) => {
        AttrInfo { attr_type: $type.to_string(), description: $desc.to_string(), values: vec![$($val.to_string()),*] }
    };
}

macro_rules! tag_attrs {
    ($map:expr, $tag:expr, { $( $attr:expr => $info:expr ),* $(,)? }) => {
        {
            let mut attrs = HashMap::new();
            $( attrs.insert($attr.to_string(), $info); )*
            $map.insert($tag.to_string(), attrs);
        }
    };
}

pub fn get_element_ui_attributes() -> HashMap<String, HashMap<String, AttrInfo>> {
    let mut map: HashMap<String, HashMap<String, AttrInfo>> = HashMap::with_capacity(80);

    tag_attrs!(map, "el-row", {
        "gutter" => attr!("attribute", "grid spacing"),
        "type" => attr!("attribute", "layout mode, you can use 'flex', works in modern browsers", "flex"),
        "justify" => attr!("attribute", "horizontal alignment of flex layout", "start", "end", "center", "space-around", "space-between"),
        "align" => attr!("attribute", "vertical alignment of flex layout", "top", "middle", "bottom"),
        "tag" => attr!("attribute", "custom element tag"),
    });

    tag_attrs!(map, "el-col", {
        "span" => attr!("attribute", "number of column the grid spans"),
        "offset" => attr!("attribute", ""),
        "push" => attr!("attribute", "number of columns that grid moves to the right"),
        "pull" => attr!("attribute", "number of columns that grid moves to the left"),
        "xs" => attr!("attribute", "<768px Responsive columns or column props object"),
        "sm" => attr!("attribute", "≥768px Responsive columns or column props object"),
        "md" => attr!("attribute", "≥992 Responsive columns or column props object"),
        "lg" => attr!("attribute", "≥1200 Responsive columns or column props object"),
        "xl" => attr!("attribute", "≥1200px Responsive columns or column props object, version >= 2"),
        "tag" => attr!("attribute", "custom element tag"),
    });

    tag_attrs!(map, "el-button", {
        "type" => attr!("attribute", "button type", "primary", "success", "warning", "danger", "info", "text"),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "plain" => attr!("attribute", "determine whether it's a plain button"),
        "loading" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "icon" => attr!("attribute", ""),
        "autofocus" => attr!("attribute", ""),
        "native-type" => attr!("attribute", "same as native button's type", "button", "submit", "reset"),
        "round" => attr!("attribute", "determine whether it's a round button, default: false"),
        "circle" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-radio", {
        "label" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "border" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "name" => attr!("attribute", "native 'name' attribute"),
    });

    tag_attrs!(map, "el-radio-group", {
        "v-model" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "fill" => attr!("attribute", "border and background color when button is active"),
        "text-color" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-radio-button", {
        "label" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-checkbox", {
        "label" => attr!("attribute", ""),
        "true-label" => attr!("attribute", "value of the checkbox if it's checked"),
        "false-label" => attr!("attribute", "value of the checkbox if it's not checked"),
        "border" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "name" => attr!("attribute", "native 'name' attribute"),
        "checked" => attr!("attribute", ""),
        "indeterminate" => attr!("attribute", "same as indeterminate in native checkbox"),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-checkbox-group", {
        "v-model" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "fill" => attr!("attribute", "border and background color when button is active"),
        "text-color" => attr!("attribute", ""),
        "min" => attr!("attribute", "minimum number of checkbox checked"),
        "max" => attr!("attribute", "maximum number of checkbox checked"),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-checkbox-button", {
        "label" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-input", {
        "v-model" => attr!("attribute", ""),
        "placeholder" => attr!("attribute", ""),
        "type" => attr!("attribute", "Same as the 'type' attribute of native input, except that it can be 'textarea'"),
        "value" => attr!("attribute", ""),
        "maxlength" => attr!("attribute", "maximum Input text length"),
        "minlength" => attr!("attribute", "minimum Input text length"),
        "disabled" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "icon" => attr!("attribute", "icon name"),
        "prefix-icon" => attr!("attribute", "prefix icon class"),
        "suffix-icon" => attr!("attribute", "suffix icon class"),
        "rows" => attr!("attribute", "number of rows of textarea, only works when type is 'textarea'"),
        "autosize" => attr!("attribute", "whether textarea has an adaptive height"),
        "auto-complete" => attr!("attribute", "", "one", "off"),
        "name" => attr!("attribute", "native 'name' attribute"),
        "readonly" => attr!("attribute", ""),
        "max" => attr!("attribute", ""),
        "min" => attr!("attribute", ""),
        "step" => attr!("attribute", "same as step in native input"),
        "resize" => attr!("attribute", "control the resizability", "none", "both", "horizontal", "vertical"),
        "autofocus" => attr!("attribute", ""),
        "form" => attr!("attribute", "same as 'form' in native input"),
        "label" => attr!("attribute", ""),
        "tabindex" => attr!("attribute", "input tabindex"),
        "clearable" => attr!("attribute", ""),
        "on-icon-click" => attr!("attribute", "hook function when clicking on the input icon"),
        "click" => attr!("method", ""),
        "blur" => attr!("method", ""),
        "focus" => attr!("method", ""),
        "change" => attr!("method", ""),
        "clear" => attr!("method", "triggers when the Input is cleared by the button which generated by the 'clearable' attribute"),
    });

    tag_attrs!(map, "el-autocomplete", {
        "v-model" => attr!("attribute", ""),
        "placeholder" => attr!("attribute", ""),
        "value" => attr!("attribute", ""),
        "debounce" => attr!("attribute", "debounce delay when typing, in milliseconds, default: 300"),
        "disabled" => attr!("attribute", ""),
        "props" => attr!("attribute", ""),
        "custom-item" => attr!("attribute", "component name of your customized suggestion list item"),
        "icon" => attr!("attribute", ""),
        "fetch-suggestions" => attr!("attribute", "a method to fetch input suggestions"),
        "popper-class" => attr!("attribute", ""),
        "trigger-on-focus" => attr!("attribute", "whether show suggestions when input focus"),
        "on-icon-click" => attr!("attribute", "hook function when clicking on the input icon"),
        "select-when-unmatched" => attr!("attribute", "whether to emit a 'select' event on enter when there is no autocomplete match"),
        "label" => attr!("attribute", ""),
        "prefix-icon" => attr!("attribute", "prefix icon class"),
        "suffix-icon" => attr!("attribute", "suffix icon class"),
        "select" => attr!("method", ""),
    });

    tag_attrs!(map, "el-input-number", {
        "v-model" => attr!("attribute", ""),
        "placeholder" => attr!("attribute", ""),
        "value" => attr!("attribute", ""),
        "min" => attr!("attribute", "the minimum allowed value"),
        "max" => attr!("attribute", "the maximum allowed value"),
        "step" => attr!("attribute", "incremental step"),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "disabled" => attr!("attribute", ""),
        "controls-position" => attr!("attribute", "position of the control buttons", "right"),
        "controls" => attr!("attribute", ""),
        "debounce" => attr!("attribute", "debounce delay when typing, in millisecond"),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-select", {
        "v-model" => attr!("attribute", ""),
        "placeholder" => attr!("attribute", ""),
        "multiple" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "value-key" => attr!("attribute", "unique identity key name for value"),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "clearable" => attr!("attribute", ""),
        "collapse-tags" => attr!("attribute", "whether to collapse tags to a text when multiple selecting"),
        "multiple-limit" => attr!("attribute", "maximum number of options user can select when multiple is true"),
        "name" => attr!("attribute", "native 'name' attribute"),
        "auto-complete" => attr!("attribute", "", "one", "off"),
        "filterable" => attr!("attribute", ""),
        "allow-create" => attr!("attribute", ""),
        "filter-method" => attr!("attribute", ""),
        "remote" => attr!("attribute", ""),
        "remote-method" => attr!("attribute", ""),
        "loading" => attr!("attribute", ""),
        "loading-text" => attr!("attribute", "displayed text while loading data from server"),
        "no-match-text" => attr!("attribute", "displayed text when no data matches the filtering query"),
        "no-data-text" => attr!("attribute", "displayed text when there is no options"),
        "popper-class" => attr!("attribute", ""),
        "reserve-keyword" => attr!("attribute", "when 'multiple' and 'filter' is true, whether to reserve current keyword after selecting an option"),
        "default-first-option" => attr!("attribute", "select first matching option on enter key"),
        "popper-append-to-body" => attr!("attribute", "whether to append the popper menu to body"),
        "change" => attr!("method", ""),
        "visible-change" => attr!("method", ""),
        "remote-tag" => attr!("attribute", ""),
        "clear" => attr!("method", "triggers when the clear icon is clicked in a clearable Select"),
        "blur" => attr!("method", ""),
        "focus" => attr!("method", ""),
    });

    tag_attrs!(map, "el-option-group", {
        "v-for" => attr!("attribute", ""),
        "key" => attr!("attribute", ""),
        "label" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-option", {
        "label" => attr!("attribute", ""),
        "value" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-cascader", {
        "options" => attr!("attribute", ""),
        "v-model" => attr!("attribute", ""),
        "props" => attr!("attribute", ""),
        "separator" => attr!("attribute", ""),
        "popper-class" => attr!("attribute", ""),
        "placeholder" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "clearable" => attr!("attribute", ""),
        "expand-trigger" => attr!("attribute", "", "click", "hover"),
        "show-all-levels" => attr!("attribute", ""),
        "filterable" => attr!("attribute", ""),
        "debounce" => attr!("attribute", ""),
        "change-on-select" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "change" => attr!("method", ""),
        "active-item-change" => attr!("method", ""),
        "blur" => attr!("method", ""),
        "focus" => attr!("method", ""),
    });

    tag_attrs!(map, "el-switch", {
        "v-model" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "width" => attr!("attribute", ""),
        "active-icon-class" => attr!("attribute", ""),
        "inactive-icon-class" => attr!("attribute", ""),
        "active-text" => attr!("attribute", ""),
        "inactive-text" => attr!("attribute", ""),
        "active-value" => attr!("attribute", ""),
        "inactive-value" => attr!("attribute", ""),
        "active-color" => attr!("attribute", ""),
        "inactive-color" => attr!("attribute", ""),
        "name" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-slider", {
        "v-model" => attr!("attribute", ""),
        "min" => attr!("attribute", ""),
        "max" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "step" => attr!("attribute", ""),
        "show-input" => attr!("attribute", ""),
        "show-stops" => attr!("attribute", ""),
        "range" => attr!("attribute", ""),
        "vertical" => attr!("attribute", ""),
        "height" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-time-picker", {
        "v-model" => attr!("attribute", ""),
        "readonly" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "editable" => attr!("attribute", ""),
        "clearable" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "placeholder" => attr!("attribute", ""),
        "is-range" => attr!("attribute", ""),
        "arrow-control" => attr!("attribute", ""),
        "picker-options" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-date-picker", {
        "v-model" => attr!("attribute", ""),
        "readonly" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "editable" => attr!("attribute", ""),
        "clearable" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "large", "small", "mini"),
        "placeholder" => attr!("attribute", ""),
        "type" => attr!("attribute", "", "year", "month", "date", "dates", "week", "datetime", "datetimerange", "daterange"),
        "format" => attr!("attribute", ""),
        "align" => attr!("attribute", "", "left", "center", "right"),
        "popper-class" => attr!("attribute", ""),
        "picker-options" => attr!("attribute", ""),
        "range-separator" => attr!("attribute", ""),
        "default-value" => attr!("attribute", ""),
        "value-format" => attr!("attribute", ""),
        "change" => attr!("method", ""),
        "blur" => attr!("method", ""),
        "focus" => attr!("method", ""),
    });

    tag_attrs!(map, "el-upload", {
        "action" => attr!("attribute", ""),
        "headers" => attr!("attribute", ""),
        "multiple" => attr!("attribute", ""),
        "data" => attr!("attribute", ""),
        "name" => attr!("attribute", ""),
        "with-credentials" => attr!("attribute", ""),
        "show-file-list" => attr!("attribute", ""),
        "drag" => attr!("attribute", ""),
        "accept" => attr!("attribute", ""),
        "on-preview" => attr!("attribute", ""),
        "on-remove" => attr!("attribute", ""),
        "on-success" => attr!("attribute", ""),
        "on-error" => attr!("attribute", ""),
        "on-progress" => attr!("attribute", ""),
        "on-change" => attr!("attribute", ""),
        "before-upload" => attr!("attribute", ""),
        "before-remove" => attr!("attribute", ""),
        "list-type" => attr!("attribute", "", "text", "picture", "picture-card"),
        "auto-upload" => attr!("attribute", ""),
        "file-list" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "limit" => attr!("attribute", ""),
        "on-exceed" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-rate", {
        "v-model" => attr!("attribute", ""),
        "max" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "allow-half" => attr!("attribute", ""),
        "low-threshold" => attr!("attribute", ""),
        "high-threshold" => attr!("attribute", ""),
        "colors" => attr!("attribute", ""),
        "show-text" => attr!("attribute", ""),
        "show-score" => attr!("attribute", ""),
        "texts" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-color-picker", {
        "v-model" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "show-alpha" => attr!("attribute", ""),
        "color-format" => attr!("attribute", "", "hsl", "hsv", "hex", "rgb"),
        "popper-class" => attr!("attribute", ""),
        "predefine" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-transfer", {
        "v-model" => attr!("attribute", ""),
        "data" => attr!("attribute", ""),
        "filterable" => attr!("attribute", ""),
        "filter-placeholder" => attr!("attribute", ""),
        "filter-method" => attr!("attribute", ""),
        "target-order" => attr!("attribute", "", "original", "push", "unshift"),
        "titles" => attr!("attribute", ""),
        "button-texts" => attr!("attribute", ""),
        "render-content" => attr!("attribute", ""),
        "format" => attr!("attribute", ""),
        "props" => attr!("attribute", ""),
        "left-default-checked" => attr!("attribute", ""),
        "right-default-checked" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-form", {
        "model" => attr!("attribute", ""),
        "rules" => attr!("attribute", ""),
        "inline" => attr!("attribute", ""),
        "label-position" => attr!("attribute", "", "right", "left", "top"),
        "label-width" => attr!("attribute", ""),
        "label-suffix" => attr!("attribute", ""),
        "hide-required-asterisk" => attr!("attribute", ""),
        "show-message" => attr!("attribute", ""),
        "inline-message" => attr!("attribute", ""),
        "status-icon" => attr!("attribute", ""),
        "validate-on-rule-change" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-form-item", {
        "prop" => attr!("attribute", ""),
        "label" => attr!("attribute", ""),
        "label-width" => attr!("attribute", ""),
        "required" => attr!("attribute", ""),
        "rules" => attr!("attribute", ""),
        "error" => attr!("attribute", ""),
        "show-message" => attr!("attribute", ""),
        "inline-message" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
    });

    tag_attrs!(map, "el-table", {
        "data" => attr!("attribute", ""),
        "height" => attr!("attribute", ""),
        "max-height" => attr!("attribute", ""),
        "stripe" => attr!("attribute", ""),
        "border" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "fit" => attr!("attribute", ""),
        "show-header" => attr!("attribute", ""),
        "highlight-current-row" => attr!("attribute", ""),
        "current-row-key" => attr!("attribute", ""),
        "row-class-name" => attr!("attribute", ""),
        "row-style" => attr!("attribute", ""),
        "row-key" => attr!("attribute", ""),
        "empty-text" => attr!("attribute", ""),
        "default-expand-all" => attr!("attribute", ""),
        "default-sort" => attr!("attribute", ""),
        "show-summary" => attr!("attribute", ""),
        "sum-text" => attr!("attribute", ""),
        "summary-method" => attr!("attribute", ""),
        "span-method" => attr!("attribute", ""),
        "select" => attr!("method", ""),
        "select-all" => attr!("method", ""),
        "selection-change" => attr!("method", ""),
        "cell-click" => attr!("method", ""),
        "row-click" => attr!("method", ""),
        "sort-change" => attr!("method", ""),
        "filter-change" => attr!("method", ""),
        "current-change" => attr!("method", ""),
        "expand-change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-table-column", {
        "type" => attr!("attribute", "", "selection", "index", "expand"),
        "index" => attr!("attribute", ""),
        "column-key" => attr!("attribute", ""),
        "label" => attr!("attribute", ""),
        "prop" => attr!("attribute", ""),
        "width" => attr!("attribute", ""),
        "min-width" => attr!("attribute", ""),
        "fixed" => attr!("attribute", "", "true", "left", "right"),
        "sortable" => attr!("attribute", "", "true", "false", "custom"),
        "sort-method" => attr!("attribute", ""),
        "sort-by" => attr!("attribute", ""),
        "resizable" => attr!("attribute", ""),
        "formatter" => attr!("attribute", ""),
        "show-overflow-tooltip" => attr!("attribute", ""),
        "align" => attr!("attribute", "", "left", "center", "right"),
        "header-align" => attr!("attribute", "", "left", "center", "right"),
        "class-name" => attr!("attribute", ""),
        "selectable" => attr!("attribute", ""),
        "filters" => attr!("attribute", ""),
        "filter-method" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-tag", {
        "type" => attr!("attribute", "", "success", "info", "warning", "danger"),
        "closable" => attr!("attribute", ""),
        "disable-transitions" => attr!("attribute", ""),
        "hit" => attr!("attribute", ""),
        "color" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "close" => attr!("method", ""),
    });

    tag_attrs!(map, "el-progress", {
        "percentage" => attr!("attribute", ""),
        "type" => attr!("attribute", "", "line", "circle"),
        "stroke-width" => attr!("attribute", ""),
        "text-inside" => attr!("attribute", ""),
        "status" => attr!("attribute", "", "success", "exception"),
        "color" => attr!("attribute", ""),
        "width" => attr!("attribute", ""),
        "show-text" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-tree", {
        "data" => attr!("attribute", ""),
        "empty-text" => attr!("attribute", ""),
        "node-key" => attr!("attribute", ""),
        "props" => attr!("attribute", ""),
        "load" => attr!("attribute", ""),
        "render-content" => attr!("attribute", ""),
        "highlight-current" => attr!("attribute", ""),
        "default-expand-all" => attr!("attribute", ""),
        "expand-on-click-node" => attr!("attribute", ""),
        "check-on-click-node" => attr!("attribute", ""),
        "auto-expand-parent" => attr!("attribute", ""),
        "show-checkbox" => attr!("attribute", ""),
        "check-strictly" => attr!("attribute", ""),
        "accordion" => attr!("attribute", ""),
        "indent" => attr!("attribute", ""),
        "lazy" => attr!("attribute", ""),
        "draggable" => attr!("attribute", ""),
        "node-click" => attr!("method", ""),
        "node-expand" => attr!("method", ""),
        "check-change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-pagination", {
        "small" => attr!("attribute", ""),
        "background" => attr!("attribute", ""),
        "page-size" => attr!("attribute", ""),
        "total" => attr!("attribute", ""),
        "page-count" => attr!("attribute", ""),
        "pager-count" => attr!("attribute", ""),
        "current-page" => attr!("attribute", ""),
        "layout" => attr!("attribute", ""),
        "page-sizes" => attr!("attribute", ""),
        "prev-text" => attr!("attribute", ""),
        "next-text" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "size-change" => attr!("method", ""),
        "current-change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-badge", {
        "value" => attr!("attribute", ""),
        "max" => attr!("attribute", ""),
        "is-dot" => attr!("attribute", ""),
        "hidden" => attr!("attribute", ""),
        "type" => attr!("attribute", "", "primary", "success", "warning", "danger", "info"),
    });

    tag_attrs!(map, "el-alert", {
        "title" => attr!("attribute", ""),
        "type" => attr!("attribute", "", "success", "warning", "info", "error"),
        "description" => attr!("attribute", ""),
        "closable" => attr!("attribute", ""),
        "center" => attr!("attribute", ""),
        "close-text" => attr!("attribute", ""),
        "show-icon" => attr!("attribute", ""),
        "close" => attr!("method", ""),
    });

    tag_attrs!(map, "el-menu", {
        "mode" => attr!("attribute", "", "horizontal", "vertical"),
        "collapse" => attr!("attribute", ""),
        "background-color" => attr!("attribute", ""),
        "text-color" => attr!("attribute", ""),
        "active-text-color" => attr!("attribute", ""),
        "default-active" => attr!("attribute", ""),
        "default-openeds" => attr!("attribute", ""),
        "unique-opened" => attr!("attribute", ""),
        "menu-trigger" => attr!("attribute", "", "hover", "click"),
        "router" => attr!("attribute", ""),
        "collapse-transition" => attr!("attribute", ""),
        "select" => attr!("method", ""),
        "open" => attr!("method", ""),
        "close" => attr!("method", ""),
    });

    tag_attrs!(map, "el-submenu", {
        "index" => attr!("attribute", ""),
        "popper-class" => attr!("attribute", ""),
        "show-timeout" => attr!("attribute", ""),
        "hide-timeout" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-menu-item", {
        "index" => attr!("attribute", ""),
        "route" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-tabs", {
        "v-model" => attr!("attribute", ""),
        "type" => attr!("attribute", "", "card", "border-card"),
        "closable" => attr!("attribute", ""),
        "addable" => attr!("attribute", ""),
        "editable" => attr!("attribute", ""),
        "tab-position" => attr!("attribute", "", "top", "right", "bottom", "left"),
        "stretch" => attr!("attribute", ""),
        "before-leave" => attr!("attribute", ""),
        "tab-click" => attr!("method", ""),
        "tab-remove" => attr!("method", ""),
        "tab-add" => attr!("method", ""),
    });

    tag_attrs!(map, "el-tab-pane", {
        "label" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "name" => attr!("attribute", ""),
        "closable" => attr!("attribute", ""),
        "lazy" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-breadcrumb", {
        "separator" => attr!("attribute", ""),
        "separator-class" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-breadcrumb-item", {
        "to" => attr!("attribute", ""),
        "replace" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-dropdown", {
        "type" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "medium", "small", "mini"),
        "split-button" => attr!("attribute", ""),
        "placement" => attr!("attribute", "", "top", "top-start", "top-end", "bottom", "bottom-start", "bottom-end"),
        "trigger" => attr!("attribute", "", "hover", "click"),
        "hide-on-click" => attr!("attribute", ""),
        "show-timeout" => attr!("attribute", ""),
        "hide-timeout" => attr!("attribute", ""),
        "click" => attr!("method", ""),
        "command" => attr!("method", ""),
        "visible-change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-dropdown-item", {
        "command" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "divided" => attr!("attribute", ""),
        "icon" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-steps", {
        "space" => attr!("attribute", ""),
        "direction" => attr!("attribute", "", "vertical", "horizontal"),
        "active" => attr!("attribute", ""),
        "process-status" => attr!("attribute", "", "wait", "process", "finish", "error", "success"),
        "finish-status" => attr!("attribute", "", "wait", "process", "finish", "error", "success"),
        "align-center" => attr!("attribute", ""),
        "simple" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-step", {
        "title" => attr!("attribute", ""),
        "description" => attr!("attribute", ""),
        "icon" => attr!("attribute", ""),
        "status" => attr!("attribute", "", "wait", "process", "finish", "error", "success"),
    });

    tag_attrs!(map, "el-dialog", {
        "visible" => attr!("attribute", ""),
        "title" => attr!("attribute", ""),
        "width" => attr!("attribute", ""),
        "fullscreen" => attr!("attribute", ""),
        "top" => attr!("attribute", ""),
        "modal" => attr!("attribute", ""),
        "modal-append-to-body" => attr!("attribute", ""),
        "append-to-body" => attr!("attribute", ""),
        "lock-scroll" => attr!("attribute", ""),
        "custom-class" => attr!("attribute", ""),
        "close-on-click-modal" => attr!("attribute", ""),
        "close-on-press-escape" => attr!("attribute", ""),
        "show-close" => attr!("attribute", ""),
        "before-close" => attr!("attribute", ""),
        "center" => attr!("attribute", ""),
        "open" => attr!("method", ""),
        "close" => attr!("method", ""),
        "opened" => attr!("method", ""),
        "closed" => attr!("method", ""),
    });

    tag_attrs!(map, "el-tooltip", {
        "effect" => attr!("attribute", "", "dark", "light"),
        "content" => attr!("attribute", ""),
        "placement" => attr!("attribute", "", "top", "top-start", "top-end", "bottom", "bottom-start", "bottom-end", "left", "left-start", "left-end", "right", "right-start", "right-end"),
        "value" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
        "offset" => attr!("attribute", ""),
        "transition" => attr!("attribute", ""),
        "popper-class" => attr!("attribute", ""),
        "open-delay" => attr!("attribute", ""),
        "manual" => attr!("attribute", ""),
        "enterable" => attr!("attribute", ""),
        "hide-after" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-popover", {
        "trigger" => attr!("attribute", "", "click", "focus", "hover", "manual"),
        "title" => attr!("attribute", ""),
        "content" => attr!("attribute", ""),
        "width" => attr!("attribute", ""),
        "placement" => attr!("attribute", "", "top", "top-start", "top-end", "bottom", "bottom-start", "bottom-end", "left", "left-start", "left-end", "right", "right-start", "right-end"),
        "disabled" => attr!("attribute", ""),
        "offset" => attr!("attribute", ""),
        "transition" => attr!("attribute", ""),
        "popper-class" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-card", {
        "header" => attr!("attribute", ""),
        "body-style" => attr!("attribute", ""),
        "shadow" => attr!("attribute", "", "always", "hover", "never"),
    });

    tag_attrs!(map, "el-carousel", {
        "height" => attr!("attribute", ""),
        "initial-index" => attr!("attribute", ""),
        "trigger" => attr!("attribute", "", "hover", "click"),
        "autoplay" => attr!("attribute", ""),
        "interval" => attr!("attribute", ""),
        "indicator-position" => attr!("attribute", "", "outside", "none"),
        "arrow" => attr!("attribute", "", "always", "hover", "never"),
        "type" => attr!("attribute", "", "card"),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-carousel-item", {
        "name" => attr!("attribute", ""),
        "label" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-collapse", {
        "v-model" => attr!("attribute", ""),
        "accordion" => attr!("attribute", ""),
        "change" => attr!("method", ""),
    });

    tag_attrs!(map, "el-collapse-item", {
        "name" => attr!("attribute", ""),
        "title" => attr!("attribute", ""),
        "disabled" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-timeline", {
        "reverse" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-timeline-item", {
        "timestamp" => attr!("attribute", ""),
        "hide-timestamp" => attr!("attribute", ""),
        "placement" => attr!("attribute", "", "top", "bottom"),
        "type" => attr!("attribute", "", "primary", "success", "warning", "danger", "info"),
        "color" => attr!("attribute", ""),
        "size" => attr!("attribute", "", "normal", "large"),
        "icon" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-divider", {
        "direction" => attr!("attribute", "", "horizontal", "vertical"),
        "content-position" => attr!("attribute", "", "left", "right", "center"),
    });

    tag_attrs!(map, "el-calendar", {
        "v-model" => attr!("attribute", ""),
        "range" => attr!("attribute", ""),
    });

    tag_attrs!(map, "el-image", {
        "src" => attr!("attribute", ""),
        "fit" => attr!("attribute", "", "fill", "contain", "cover", "none", "scale-down"),
        "alt" => attr!("attribute", ""),
        "referrer-policy" => attr!("attribute", ""),
        "lazy" => attr!("attribute", ""),
        "scroll-container" => attr!("attribute", ""),
        "preview-src-list" => attr!("attribute", ""),
        "z-index" => attr!("attribute", ""),
        "load" => attr!("method", ""),
        "error" => attr!("method", ""),
    });

    tag_attrs!(map, "el-backtop", {
        "target" => attr!("attribute", ""),
        "visibility-height" => attr!("attribute", ""),
        "right" => attr!("attribute", ""),
        "bottom" => attr!("attribute", ""),
        "click" => attr!("method", ""),
    });

    tag_attrs!(map, "el-drawer", {
        "append-to-body" => attr!("attribute", ""),
        "before-close" => attr!("attribute", ""),
        "close-on-press-escape" => attr!("attribute", ""),
        "custom-class" => attr!("attribute", ""),
        "destroy-on-close" => attr!("attribute", ""),
        "modal" => attr!("attribute", ""),
        "modal-append-to-body" => attr!("attribute", ""),
        "direction" => attr!("attribute", "", "rtl", "ltr", "ttb", "btt"),
        "show-close" => attr!("attribute", ""),
        "size" => attr!("attribute", ""),
        "title" => attr!("attribute", ""),
        "visible" => attr!("attribute", ""),
        "wrapperClosable" => attr!("attribute", ""),
        "open" => attr!("method", ""),
        "opened" => attr!("method", ""),
        "close" => attr!("method", ""),
        "closed" => attr!("method", ""),
    });

    map
}
