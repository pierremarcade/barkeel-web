import { beforeSubmit, handleSelectAndRadioElements, autocomplete } from './form.js';
import { init } from './quill.js';

document.addEventListener('DOMContentLoaded', function() {
    beforeSubmit();
    handleSelectAndRadioElements();
    autocomplete();
    init();
});
