import { beforeSubmit, handleSelectAndRadioElements, handleFileElements, handleAutocompleteElements, handleCheckboxElements } from './form.js';
import { init } from './quill.js';

document.addEventListener('DOMContentLoaded', function() {
    handleCheckboxElements();
    handleSelectAndRadioElements();
    handleFileElements();
    handleAutocompleteElements();
    init();
    beforeSubmit();
});
