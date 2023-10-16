export function initDatePicker(selector) {
    const elem = document.querySelector(selector);
    const datePicker = new Datepicker(elem, { buttonClass: 'btn' });
}