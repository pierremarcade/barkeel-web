document.addEventListener('DOMContentLoaded', function() {
    handleSelectAndRadioElements();
    tinymce.init({
        selector: 'textarea',
        plugins: 'textpattern anchor autolink charmap codesample emoticons image link lists media searchreplace table visualblocks wordcount checklist mediaembed casechange export formatpainter pageembed linkchecker a11ychecker tinymcespellchecker permanentpen powerpaste advtable advcode editimage advtemplate ai mentions tinycomments tableofcontents footnotes mergetags autocorrect typography inlinecss markdown',
        toolbar: 'undo redo | blocks fontfamily fontsize | bold italic underline strikethrough | link image media table mergetags | addcomment showcomments | spellcheckdialog a11ycheck typography | align lineheight | checklist numlist bullist indent outdent | emoticons charmap | removeformat',
        tinycomments_mode: 'embedded',
        ai_request: (request, respondWith) => respondWith.string(() => Promise.reject("See docs to implement AI Assistant")),
    });
});

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
    radioElement.innerHTML = ''; // Vider le contenu actuel
    Object.keys(data).forEach(function(key) {
        var name = radioElement.getAttribute('data-name');
        var value = data[key][radioElement.getAttribute('data-id')];
        var labelText = data[key][radioElement.getAttribute('data-label')];
        var selected = radioElement.getAttribute('data-selected');

        var radioHtml = createRadioInput(name, value, labelText, selected);
        radioElement.innerHTML += radioHtml; // Ajouter le nouveau radio
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