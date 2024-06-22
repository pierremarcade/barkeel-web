import { createSlug } from './utils.js';

export function init() {
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
        var slug = createSlug(titleElement.textContent);
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
            var slug = createSlug(titleElement.textContent);
            titleElement.id = slug;
        });
        if (source === 'user') {
            textarea.value = quill.root.innerHTML;
        }
    });
});
}