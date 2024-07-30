"use strict";function t(t,e){fetch(t,{method:"GET",headers:{"Content-Type":"application/json"}}).then((function(t){if(!t.ok)throw new Error("Fetch options error");return t.json()})).then(e).catch((function(t){console.error("Fetch options error:",t)}))}function e(t){return t.toLowerCase().replace(/[^a-z0-9]+/g,"-").replace(/(^-|-$)/g,"")}async function n(t){const e=t.target.files;if(0===e.length)return;const n=new FormData;n.append("file",e[0]);const o=t.target.dataset.url;try{const t=await fetch(o,{method:"POST",body:n});if(!t.ok)throw new Error(`HTTP error status: ${t.status}`);console.log("File uploaded with success")}catch(t){console.error("An error occured during uploading file :",t)}}function o(){var e=document.querySelectorAll(".select"),n=document.querySelectorAll(".radio");e.forEach((function(e){t(e.getAttribute("data-url"),(function(t){!function(t,e){t.innerHTML="",Object.keys(e).forEach((function(n){var o=document.createElement("option");o.value=e[n][t.getAttribute("data-id")],o.textContent=e[n][t.getAttribute("data-label")],t.appendChild(o)}));var n=t.getAttribute("data-selected");n&&(t.value=n)}(e,t)}))})),n.forEach((function(e){t(e.getAttribute("data-url"),(function(t){!function(t,e){t.innerHTML="",Object.keys(e).forEach((function(n){var o=function(t,e,n,o){return`\n        <div class="sm:col-span-4">\n            <input id="${t}" name="${t}" value="${e}" ${o==e?"checked":""} type="radio"\n                class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-600">\n            <label for="${t}"\n                class="block text-sm font-medium leading-6 text-gray-900">${n}</label>\n        </div>\n    `}(t.getAttribute("data-name"),e[n][t.getAttribute("data-id")],e[n][t.getAttribute("data-label")],t.getAttribute("data-selected"));t.innerHTML+=o}))}(e,t)}))}))}function r(){const e=document.querySelectorAll(".autocomplete"),n=document.querySelector("form");if(n){document.querySelectorAll(".autocomplete-selected-items").forEach((function(t){t.innerHTML=""}));let o=[];e.forEach((e=>{e.addEventListener("input",(function(){const o=this.getAttribute("id"),r=n.querySelector(`#${o}List`);r&&(r.innerHTML="",r.style.display="block",this.value.length>=3?(r.style.display="block",t(`${this.getAttribute("data-url")}/${this.value}`,(function(t){Object.keys(t).forEach((function(n){const o=document.createElement("li");o.textContent=t[n][e.getAttribute("data-label")],o.setAttribute("id",t[n][e.getAttribute("data-id")]),o.classList.add("p-2","hover:bg-gray-100","cursor-pointer"),r.appendChild(o)}))}))):r.style.display="none")}))}));const r=n.querySelector(".autocomplete-list");r&&r.addEventListener("click",(function(t){if("li"!==t.target.tagName.toLowerCase())return;r.style.display="none";const e=t.target.getAttribute("id");if(o.includes(e))return;o.push(e);const c=t.target.parentElement.getAttribute("data-id"),i=document.createElement("input");i.id=`${e}-autocomplete-selected`,i.setAttribute("type","checkbox"),i.setAttribute("style","display:none"),i.setAttribute("name",`${c}`),i.setAttribute("value",e),i.setAttribute("checked","checked");const l=n.querySelector(`#${c}Selected`);if(l){const n=document.createElement("span");n.className="\n                        remove-from-list\n                        inline-flex \n                        items-center\n                        gap-x-1.5 \n                        rounded-md \n                        bg-indigo-600 \n                        px-3 \n                        py-2 \n                        text-sm \n                        font-semibold \n                        text-white \n                        shadow-sm \n                        hover:bg-indigo-500 \n                        focus-visible:outline \n                        focus-visible:outline-2 \n                        focus-visible:outline-offset-2 \n                        focus-visible:outline-indigo-600\n                        ",n.textContent=t.target.textContent,n.appendChild(function(){let t=document.createElementNS("http://www.w3.org/2000/svg","svg");t.setAttribute("viewBox","0 0 24 24"),t.setAttribute("width","15"),t.setAttribute("height","15"),t.setAttribute("className","-mr-0.5 h-5 w-5 cursor-pointer");let e=document.createElementNS("http://www.w3.org/2000/svg","path");return e.setAttribute("d","M22.245,4.015c0.313,0.313,0.313,0.826,0,1.139l-6.276,6.27c-0.313,0.312-0.313,0.826,0,1.14l6.273,6.272  c0.313,0.313,0.313,0.826,0,1.14l-2.285,2.277c-0.314,0.312-0.828,0.312-1.142,0l-6.271-6.271c-0.313-0.313-0.828-0.313-1.141,0  l-6.276,6.267c-0.313,0.313-0.828,0.313-1.141,0l-2.282-2.28c-0.313-0.313-0.313-0.826,0-1.14l6.278-6.269  c0.313-0.312,0.313-0.826,0-1.14L1.709,5.147c-0.314-0.313-0.314-0.827,0-1.14l2.284-2.278C4.308,1.417,4.821,1.417,5.135,1.73  L11.405,8c0.314,0.314,0.828,0.314,1.141,0.001l6.276-6.267c0.312-0.312,0.826-0.312,1.141,0L22.245,4.015z"),t.appendChild(e),t}()),l.appendChild(n),l.appendChild(i),n.addEventListener("click",(function(){l.removeChild(i),l.removeChild(n),o=o.filter((t=>t!==e))}))}}))}}document.addEventListener("DOMContentLoaded",(function(){document.querySelectorAll(".checkbox").forEach((function(t){"true"===t.getAttribute("data-checked")&&(t.checked=!0)})),o(),function(){const t=document.querySelectorAll('input[type="file"][data-url]');t&&t.forEach((t=>{t.addEventListener("change",n)}))}(),r(),function(){const t=Quill.import("blots/block"),n=Quill.import("blots/container"),o=Quill.import("blots/break"),r=Quill.import("blots/text"),c=Quill.import("blots/cursor");class i extends n{static create(t){const e=super.create(t);return e.setAttribute("spellcheck","false"),e.setAttribute("class","prism-code ql-code-block-container "),e}}class l extends t{static TAB="  ";static register(){Quill.register(i)}}i.blotName="code-block-container",i.tagName="pre",i.allowedChildren=[l],l.blotName="code-block",l.className="ql-code-block",l.tagName="DIV",l.allowedChildren=[r,o,c],l.requiredContainer=i,Quill.register(l),document.querySelectorAll(".editor").forEach((function(t){var n=t.id,o=document.querySelector(`textarea[data-editor-id="${n}"]`);o.querySelectorAll("h1, h2, h3, h4, h5, h6").forEach((function(t){var n=e(t.textContent);t.id=n}));var r=o?o.value:"",c=new Quill(t,{theme:"snow",modules:{syntax:!0,toolbar:[["bold","italic","underline","strike"],["blockquote","code-block"],[{list:"ordered"},{list:"bullet"}],[{script:"sub"},{script:"super"}],[{indent:"-1"},{indent:"+1"}],[{direction:"rtl"}],[{size:["small",!1,"large","huge"]}],[{header:[1,2,3,4,5,6,!1]}],[{color:[]},{background:[]}],[{font:[]}],[{align:[]}],["clean"]]}});""!==r&&(c.root.innerHTML=r),c.on("text-change",(function(t,n,r){c.root.querySelectorAll("h1, h2, h3, h4, h5, h6").forEach((function(t){var n=e(t.textContent);t.id=n})),"user"===r&&(o.value=c.root.innerHTML)}))}))}(),function(){const t=document.querySelectorAll("form");t&&t.forEach((function(t){const n=document.querySelectorAll('input[type="datetime-local"]');n.forEach((function(t){var e=document.querySelector(`input[data-datetime="${t.id}"]`);e&&(t.value=e.value)}));const o=document.querySelectorAll('input[type="checkbox"]');t.addEventListener("submit",(function(r){r.preventDefault(),console.log(o),o.forEach((function(t){console.log(t.value),t.checked||(t.value=!1,t.checked=!0)})),t.querySelectorAll("h1, h2, h3, h4, h5, h6").forEach((function(t){var n=e(t.textContent);t.setAttribute("id",n)})),n.forEach((function(t){var e=document.querySelector(`input[data-datetime="${t.id}"]`);let n=new Date(t.value);e.value=n.toISOString().slice(0,19),console.log(e.value)})),t.submit()}))}))}()}));
