import { beforeSubmit, handleSelectAndRadioElements, handleFileElements, handleAutocompleteElements, handleCheckboxElements } from './form.js';
import { init } from './quill.js';

function updateUrlParameter(key, value) {
    const urlParams = new URLSearchParams(window.location.search);
    const params = [];

    if (urlParams.has('order')) {
        const orders = urlParams.get('order').split(',');
        let found = false;
        orders.forEach(order => {
            const lastUnderscoreIndex = order.lastIndexOf('_');
            const col = order.substring(0, lastUnderscoreIndex);
            if (col === key) {
                found = true;
                params.push(`${key}_${value}`);
            } else {
                params.push(order);
            }
        });
        if (!found) {
            params.push(`${key}_${value}`);
        }
    } else {
        params.push(`${key}_${value}`);
    }

    // Update the 'order' parameter
    if (params.length > 0) {
        urlParams.set('order', params.join(','));
    } else {
        urlParams.delete('order');
    }

    // Rebuild the URL
    const newUrlParams = urlParams.toString();
    const currentUrlParams = window.location.search.substring(1);

    // Redirect if necessary
    if (newUrlParams !== currentUrlParams) {
        window.location.href = `${window.location.pathname}?${newUrlParams}`;
    }
}

document.addEventListener('DOMContentLoaded', function () {
    initializeTableHeaders();
    handleCheckboxElements();
    handleSelectAndRadioElements();
    handleFileElements();
    handleAutocompleteElements();
    init();
    beforeSubmit();

    document.querySelectorAll('th[data-sort]').forEach(th => {
        th.addEventListener('click', handleColumnClick);
    });
});

function handleColumnClick(event) {
    const th = event.target.closest('th[data-sort]');
    if (!th) return;

    const columnName = th.dataset.sort;
    const currentOrder = th.dataset.order || 'none';

    let newOrder;
    if (currentOrder === 'asc') {
        newOrder = 'desc';
    } else if (currentOrder === 'desc') {
        newOrder = 'none';
    } else {
        newOrder = 'asc';
    }

    handleSortParameter(columnName, newOrder);


    th.dataset.order = newOrder;
}

function handleSortParameter(columnName, newOrder) {
    if (newOrder === 'none') {
        removeUrlParameter(columnName);
    } else {
        updateUrlParameter(columnName, newOrder);
    }
}

function initializeTableHeaders() {
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.has('order')) {
        const orders = urlParams.get('order').split(',');
        orders.forEach(order => {
            const lastUnderscoreIndex = order.lastIndexOf('_');
            const col = order.substring(0, lastUnderscoreIndex);
            const val = order.substring(lastUnderscoreIndex + 1);
            const th = document.querySelector(`th[data-sort="${col}"]`);
            if (th) {
                th.dataset.order = val;
                // Dispatch a custom event to update Alpine.js state
                console.log('dispatching', `${col}-order-change`, val);
                window.dispatchEvent(new CustomEvent(`${col}-order-change`, {
                    detail: { order: val }
                }));
            }
        });
    }
}


function removeUrlParameter(key) {
    const urlParams = new URLSearchParams(window.location.search);

    if (urlParams.has('order')) {
        const orders = urlParams.get('order').split(',').filter(order => {
            const col = order.substring(0, order.lastIndexOf('_'));
            return col !== key;
        });

        if (orders.length > 0) {
            urlParams.set('order', orders.join(','));
        } else {
            urlParams.delete('order');
        }
    }

    // Rebuild the URL
    const newUrl = `${window.location.pathname}?${urlParams.toString()}`;

    // Redirect to the new URL
    window.location.href = newUrl;
}

