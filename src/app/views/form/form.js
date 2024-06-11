document.addEventListener('DOMContentLoaded', function() {
    changeDateTimeLocalFormat();
    handleSelectAndRadioElements();
    const Block = Quill.import('blots/block');
    const Container = Quill.import('blots/container');
    const Break = Quill.import('blots/break');
    const TextBlot = Quill.import('blots/text');
    const Cursor = Quill.import('blots/cursor');

    class CodeBlockContainer extends Container {
        static create(value) {
            const domNode = super.create(value) ;
            domNode.setAttribute('spellcheck', 'false');
            domNode.setAttribute('class', 'prism-code ql-code-block-container ');
            return domNode;
        }
    }
    
    class CodeBlock extends Block {
        static TAB = '  ';
        static register() {
          Quill.register(CodeBlockContainer);
        }
    }

    CodeBlockContainer.blotName = 'code-block-container';
    CodeBlockContainer.tagName = 'pre';
    CodeBlockContainer.allowedChildren = [CodeBlock];

    CodeBlock.blotName = 'code-block';
    CodeBlock.className = 'ql-code-block';
    CodeBlock.tagName = 'DIV';
    CodeBlock.allowedChildren = [TextBlot, Break, Cursor];
    CodeBlock.requiredContainer = CodeBlockContainer;
      
    Quill.register(CodeBlock);

    var editorElements = document.querySelectorAll('.editor');
    editorElements.forEach(function(editorElement) {
        var editorId = editorElement.id;
        var textarea = document.querySelector(`textarea[data-editor-id="${editorId}"]`);
        textarea.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
            var slug = createSlug(titleElement.textContent); // Assurez-vous que cette fonction est définie quelque part
            titleElement.id = slug;
        });
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
                    ['clean'],
                ],
               
            }
        });
        if (initialContent !== '') {
            quill.root.innerHTML = initialContent;
        }
        quill.on('text-change', function(delta, oldDelta, source) {
            var root = quill.root
            root.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
                var slug = createSlug(titleElement.textContent); // Assurez-vous que cette fonction est définie quelque part
                titleElement.id = slug;
            });
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
        var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
        field.value = datetime_field.value;
    });

    form.addEventListener('submit', function(event) {
        event.preventDefault();
        console.log(form)
        form.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
            var slug = createSlug(titleElement.textContent);
            titleElement.setAttribute('id', slug);
        });

        datetimeFields.forEach(function(field) {
            var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
            let date = new Date(field.value);
            datetime_field.value = date.toISOString().slice(0, 19);
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

function createSlug(text) {
    return text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '');
}
