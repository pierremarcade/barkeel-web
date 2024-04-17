document.addEventListener('DOMContentLoaded', function() {
    changeDateTimeLocalFormat();
    handleSelectAndRadioElements();

    var editorElements = document.querySelectorAll('.editor');
    editorElements.forEach(function(editorElement) {
        var editorId = editorElement.id;
        var textarea = document.querySelector(`textarea[data-editor-id="${editorId}"]`);
        var initialContent = textarea ? textarea.value : '';
        var quill = new Quill(editorElement, {
            theme: 'snow',
            modules: {
                syntax: true, 
                toolbar: [
                    ['bold', 'italic', 'underline', 'strike'],
                    ['blockquote', 'code-block'],
                    [{ 'list': 'ordered'}, { 'list': 'bullet' }],
                    [{ 'script': 'sub'}, { 'script': 'super' }],
                    [{ 'indent': '-1'}, { 'indent': '+1' }],
                    [{ 'direction': 'rtl' }],
                    [{ 'size': ['small', false, 'large', 'huge'] }],
                    [{ 'header': [1, 2, 3, 4, 5, 6, false] }],
                    [{ 'color': [] }, { 'background': [] }],
                    [{ 'font': [] }],
                    [{ 'align': [] }],
                    ['clean']
                ]
            }
        });
        if (initialContent !== '') {
            quill.root.innerHTML = initialContent;
        }
        
        quill.on('text-change', function(delta, oldDelta, source) {
            if (source === 'user') {
                textarea.value = quill.root.innerHTML;
            }
        });
    });

});

function changeDateTimeLocalFormat() {
    const form = document.querySelector('form');
    const datetimeFields = form.querySelectorAll('input[type="datetime-local"]');
    datetimeFields.forEach(function(field) {
        console.log(field.id)
        var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
        field.value = datetime_field.value;
    });
    form.addEventListener('submit', function(event) {
        event.preventDefault();
        datetimeFields.forEach(function(field) {
            var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
            let date = new Date(field.value);
            datetime_field.value = date.toISOString().slice(0, 19);
            console.log(datetime_field.value)
        });
        form.submit();
    });
}

function handleSelectAndRadioElements() {
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

function fetchData(url, processData) {
    fetch(url, {
        method: 'GET', 
        headers: {
            'Content-Type': 'application/json' 
        }
    })
    .then(function(response) {
        if (!response.ok) {
            throw new Error('Fetch options error');
        }
        return response.json();
    })
    .then(processData)
    .catch(function(error) {
        console.error('Fetch options error:', error);
    });
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