import { fetchData, crossSvg, createSlug } from './utils.js';

export function beforeSubmit() {
    const forms = document.querySelectorAll('form');
    if (forms) {
        forms.forEach(function(form) {
            const datetimeFields = document.querySelectorAll('input[type="datetime-local"]');
            datetimeFields.forEach(function(field) {
                var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
                if (datetime_field) {
                    field.value = datetime_field.value;
                }
            });
            const checkboxFields = document.querySelectorAll('input[type="checkbox"]');
            form.addEventListener('submit', function(event) {
                event.preventDefault();
                console.log(checkboxFields);
                checkboxFields.forEach(function(field) {
                    console.log(field.value);
                    if (!field.checked) {
                        field.value = false;
                        field.checked = true;
                    }
                });
                form.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
                    var slug = createSlug(titleElement.textContent);
                    titleElement.setAttribute('id', slug);
                });
                datetimeFields.forEach(function(field) {
                    var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
                    let date = new Date(field.value);
                    datetime_field.value = date.toISOString().slice(0, 19);
                    console.log(datetime_field.value)
                });
                form.submit();
        });
        });
    }
}

export function handleFileElements() {
    const fileInputs = document.querySelectorAll('input[type="file"][data-url]');
    if (fileInputs) {
        fileInputs.forEach(input => {
            input.addEventListener('change', handleFileUpload);
        });  
    }
}

async function handleFileUpload(event) {
    const files = event.target.files;
    if (files.length === 0) return;
    const formData = new FormData();
    formData.append('file', files[0]);
    const url = event.target.dataset.url;
    try {
        const response = await fetch(url, {
            method: 'POST',
            body: formData,
        });
        if (!response.ok) throw new Error(`HTTP error status: ${response.status}`);
        console.log('File uploaded with success');
    } catch (error) {
        console.error('An error occured during uploading file :', error);
    }
}

export function handleSelectAndRadioElements() {
    var selectElements = document.querySelectorAll('.select');
    var radioElements = document.querySelectorAll('.radio');
    selectElements.forEach(function(selectElement) {
        fetchData(selectElement.getAttribute('data-url'), function(data) {
            populateSelectOptions(selectElement, data);
        });
    });
    radioElements.forEach(function(radioElement) {
        fetchData(radioElement.getAttribute('data-url'), function(data) {
            populateRadioOptions(radioElement, data);
        });
    });
}

export function handleCheckboxElements() {
    var checkboxes = document.querySelectorAll('.checkbox');
    checkboxes.forEach(function(checkbox) {
        if (checkbox.getAttribute('data-checked') === 'true') {
            checkbox.checked = true;
        }
    });
}

export function handleAutocompleteElements() {
    const autocompleteFields = document.querySelectorAll('.autocomplete');
    const form = document.querySelector('form');
    if (form) {
        const autocompleteSelectedItems = document.querySelectorAll('.autocomplete-selected-items');
        autocompleteSelectedItems.forEach(function(element) {
            element.innerHTML = '';
        });
        let itemsSelected = [];
        autocompleteFields.forEach(field => {
            field.addEventListener('input', function() {
                const id = this.getAttribute('id');
                const multiSelectSuggestionsList = form.querySelector(`#${id}List`);
                if (multiSelectSuggestionsList) {
                    multiSelectSuggestionsList.innerHTML = '';
                    multiSelectSuggestionsList.style.display = "block";
                    if (this.value.length >= 3) {
                        multiSelectSuggestionsList.style.display = "block";
                        fetchData(`${this.getAttribute('data-url')}/${this.value}`, function(data) {
                            Object.keys(data).forEach(function(key) {
                                const item = document.createElement('li');
                                item.textContent = data[key][field.getAttribute('data-label')];
                                item.setAttribute('id', data[key][field.getAttribute('data-id')]);
                                item.classList.add('p-2', 'hover:bg-gray-100', 'cursor-pointer');
                                multiSelectSuggestionsList.appendChild(item);
                            });
                        });
                    } else {
                        multiSelectSuggestionsList.style.display = "none";
                    }
                }
            });
        });
        const multiSelectSuggestionsList = form.querySelector(`.autocomplete-list`);
        if (multiSelectSuggestionsList) {
            multiSelectSuggestionsList.addEventListener('click', function(event) {
                if (event.target.tagName.toLowerCase()!== 'li') return;
                multiSelectSuggestionsList.style.display = "none";
                const selectedId = event.target.getAttribute('id');
                if (itemsSelected.includes(selectedId)) {
                    return;
                }
                itemsSelected.push(selectedId);
                const parentElementDataId = event.target.parentElement.getAttribute('data-id');
                const checkbox = document.createElement('input');
                checkbox.id = `${selectedId}-autocomplete-selected`;
                checkbox.setAttribute("type", "checkbox");
                checkbox.setAttribute("style", "display:none");
                checkbox.setAttribute("name", `${parentElementDataId}`);
                checkbox.setAttribute("value", selectedId);
                checkbox.setAttribute("checked", 'checked');
                const selectedItemContainer = form.querySelector(`#${parentElementDataId}Selected`);
                if (selectedItemContainer) {
                    const removeBtn = document.createElement('span');
                    removeBtn.className = `
                        remove-from-list
                        inline-flex 
                        items-center
                        gap-x-1.5 
                        rounded-md 
                        bg-indigo-600 
                        px-3 
                        py-2 
                        text-sm 
                        font-semibold 
                        text-white 
                        shadow-sm 
                        hover:bg-indigo-500 
                        focus-visible:outline 
                        focus-visible:outline-2 
                        focus-visible:outline-offset-2 
                        focus-visible:outline-indigo-600
                        `;
                    removeBtn.textContent = event.target.textContent;
                    removeBtn.appendChild(crossSvg());
                    selectedItemContainer.appendChild(removeBtn);
                    selectedItemContainer.appendChild(checkbox);
                    removeBtn.addEventListener('click', function() {
                        selectedItemContainer.removeChild(checkbox);
                        selectedItemContainer.removeChild(removeBtn);
                        itemsSelected = itemsSelected.filter(element => element!== selectedId);
                    });     
                }
            });
        }
    }
}

function createRadioInput(name, value, labelText, selected) {
    return `
        <div class="sm:col-span-4">
            <input id="${name}" name="${name}" value="${value}" ${selected == value ? 'checked' : ''} type="radio"
                class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-600">
            <label for="${name}"
                class="block text-sm font-medium leading-6 text-gray-900">${labelText}</label>
        </div>
    `;
}

function populateSelectOptions(selectElement, data) {
    selectElement.innerHTML = '';
    Object.keys(data).forEach(function(key) {
        var option = document.createElement('option');
        option.value = data[key][selectElement.getAttribute('data-id')];
        option.textContent = data[key][selectElement.getAttribute('data-label')];
        selectElement.appendChild(option);
    });
    var selectedValue = selectElement.getAttribute('data-selected');
    if (selectedValue) {
        selectElement.value = selectedValue;
    }
}

function populateRadioOptions(radioElement, data) {
    radioElement.innerHTML = '';
    Object.keys(data).forEach(function(key) {
        var name = radioElement.getAttribute('data-name');
        var value = data[key][radioElement.getAttribute('data-id')];
        var labelText = data[key][radioElement.getAttribute('data-label')];
        var selected = radioElement.getAttribute('data-selected');
        var radioHtml = createRadioInput(name, value, labelText, selected);
        radioElement.innerHTML += radioHtml;
    });
}