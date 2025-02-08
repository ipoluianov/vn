<script setup>
import { ref, reactive } from 'vue'

// import { GetFilePanelContentAsJson, GoBack, MainAction, SetCurrentItemIndex, UpdateContent } from '../../wailsjs/go/main/App'

const data = reactive(
    {
        content: null,
    }
)

const props = defineProps({
    isActive: Boolean,
    panelIndex: Number,
    panelHeight: Number,
    panelWidth: Number,
})

const emit = defineEmits(['custom-event']);

const settings = {
    headerHeight: 32,
    rowHeight: 25,
    footerHeight: 32
};

let paddingLeft = 3;
let paddingRight = 3;
let paddingTop = 2;
let paddingBottom = 2;
let paddingLeftRight = (paddingLeft + paddingRight) * 4;


//////////////////////////////////////////////////////////////////////
// EVENTS
//////////////////////////////////////////////////////////////////////
// CLICK
const onClickItem = (index) => {
    if (document.dialogIsOpen == true) {
        return
    }

    emit('activate-panel', props.panelIndex);
    setCurrentItemIndex(index);
}

// DOUBLE_CLICK
const onDblClickItem = async (index) => {
    if (document.dialogIsOpen == true) {
        return
    }

    emit('activate-panel', props.panelIndex);
    setCurrentItemIndex(index);
    await MainAction(props.panelIndex);
    await loadContent();
}

// KEYDOWN
window.addEventListener('keydown', async (event) => {

    if (props.isActive === false) {
        return
    }

    if (event.key === 'ArrowDown') {
        event.preventDefault();
        setCurrentItemIndex(data.content.currentItemIndex + 1);
    }
    if (event.key === 'ArrowUp') {
        event.preventDefault();
        setCurrentItemIndex(data.content.currentItemIndex - 1);
    }
    if (!event.altKey && event.key == 'F2') {
        event.preventDefault();
        await UpdateContent(props.panelIndex);
        await loadContent();
    }


    if (event.key == 'Enter') {
        event.preventDefault();
        await MainAction(props.panelIndex);
        await loadContent();
    }
    if (event.key == 'Backspace') {
        event.preventDefault();
        await GoBack(props.panelIndex);
        await loadContent();
    }
    if (event.key == 'PageUp') {
        event.preventDefault();
        setCurrentItemIndex(data.content.currentItemIndex - tableVisibleItemsCount());
    }
    if (event.key == 'PageDown') {
        event.preventDefault();
        setCurrentItemIndex(data.content.currentItemIndex + tableVisibleItemsCount());
    }
    if (event.key == 'Home') {
        event.preventDefault();
        setCurrentItemIndex(0);
    }
    if (event.key == 'End') {
        event.preventDefault();
        setCurrentItemIndex(data.content.items.length - 1);
    }

    //console.log(event.key);
});

/*EventsOn('updateContent', async (panelIndex) => {
    if (panelIndex != props.panelIndex) {
        return
    }
    await loadContent();
});*/

//////////////////////////////////////////////////////////////////////

const setCurrentItemIndex = async (index) => {
    //console.log("setCurrentItemIndex", '[' + props.panelIndex + ']', index, " isActive:", props.isActive);
    await SetCurrentItemIndex(props.panelIndex, index);
    await loadContent();
}

const loadContent = async () => {
    /*const content = await GetFilePanelContentAsJson(props.panelIndex);
    data.content = JSON.parse(content);
    //console.log("loadContent", data.content.currentItemIndex);
    setTimeout(() => {
        scrollToRow(idForRow(data.content.currentItemIndex));
    }, 10);*/

    data.content = {
        currentItemIndex: 0,
        currentPath: 'C:\\',
        items: [
            { displayName: 'file1.txt', displayExt: 'txt', size: '1KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file1.txt', linkPath: 'C:\\file1.txt' },
            { displayName: 'file2.txt', displayExt: 'txt', size: '2KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file2.txt', linkPath: 'C:\\file2.txt' },
            { displayName: 'file3.txt', displayExt: 'txt', size: '3KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file3.txt', linkPath: 'C:\\file3.txt' },
            { displayName: 'file4.txt', displayExt: 'txt', size: '4KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file4.txt', linkPath: 'C:\\file4.txt' },
            { displayName: 'file5.txt', displayExt: 'txt', size: '5KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file5.txt', linkPath: 'C:\\file5.txt' },
            { displayName: 'file6.txt', displayExt: 'txt', size: '6KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file6.txt', linkPath: 'C:\\file6.txt' },
            { displayName: 'file7.txt', displayExt: 'txt', size: '7KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file7.txt', linkPath: 'C:\\file7.txt' },
            { displayName: 'file8.txt', displayExt: 'txt', size: '8KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file8.txt', linkPath: 'C:\\file8.txt' },
            { displayName: 'file9.txt', displayExt: 'txt', size: '9KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file9.txt', linkPath: 'C:\\file9.txt' },
            { displayName: 'file10.txt', displayExt: 'txt', size: '10KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file10.txt', linkPath: 'C:\\file10.txt' },
            { displayName: 'file11.txt', displayExt: 'txt', size: '11KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file11.txt', linkPath: 'C:\\file11.txt' },
            { displayName: 'file12.txt', displayExt: 'txt', size: '12KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file12.txt', linkPath: 'C:\\file12.txt' },
            { displayName: 'file13.txt', displayExt: 'txt', size: '13KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file13.txt', linkPath: 'C:\\file13.txt' },
            { displayName: 'file14.txt', displayExt: 'txt', size: '14KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file14.txt', linkPath: 'C:\\file14.txt' },
            { displayName: 'file15.txt', displayExt: 'txt', size: '15KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file15.txt', linkPath: 'C:\\file15.txt' },
            { displayName: 'file16.txt', displayExt: 'txt', size: '16KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file16.txt', linkPath: 'C:\\file16.txt' },
            { displayName: 'file17.txt', displayExt: 'txt', size: '17KB', datetime: '2021-01-01 12:00:00', fullPath: 'C:\\file17.txt', linkPath: 'C:\\file17.txt' },
    ]
        }
}

const idForRow = (index) => {
    return 'panel_' + props.panelIndex + '_row_' + index;
}

const scrollToRow = (rowId) => {
    scrollToRowIfNotVisible(rowId);
    return;
    const element = document.getElementById(rowId);
    if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
}

const scrollToRowIfNotVisible = (rowId, behavior = 'instant') => {
    const element = document.getElementById(rowId);
    if (element) {
        const tableContainer = document.getElementById(tableContainerId());
        const elementRect = element.getBoundingClientRect();
        const containerRect = tableContainer.getBoundingClientRect();
        const thead = document.querySelector('thead');
        const headerHeight = thead ? thead.offsetHeight : 0;

        const isElementVisible = (
            elementRect.top >= containerRect.top + headerHeight &&
            elementRect.bottom <= containerRect.bottom
        );

        if (!isElementVisible) {
            let scrollOffset = 0;

            if (elementRect.top < containerRect.top + headerHeight) {
                scrollOffset = elementRect.top - containerRect.top - headerHeight;
            } else if (elementRect.bottom > containerRect.bottom) {
                scrollOffset = elementRect.bottom - containerRect.bottom;
            }

            tableContainer.scrollBy({ top: scrollOffset, behavior });
            console.log("SCROLL to", rowId);
        }
    }
};


const tableContainerId = () => {
    return 'table-container-' + props.panelIndex;
}

const tableVisibleItemsCount = () => {
    let rowHeight = settings.rowHeight + paddingBottom + paddingTop + 1;
    let tableHeight = props.panelHeight - settings.headerHeight - settings.footerHeight - rowHeight;
    return Math.round(tableHeight / rowHeight) - 1;
}

const currentItem = () => {
    return data.content.items[data.content.currentItemIndex];
}

const styleForItem = (index) => {
    if (props.isActive == true) {
        return {
            backgroundColor: index === data.content.currentItemIndex ? '#444' : '#00000000'
        }
    }
    return {}
}

const styleForHeader = () => {
    return {
        height: '30px',
        backgroundColor: props.isActive ? '#444' : '#000'
    }
}

const styleForFooter = () => {
    return {
        height: '30px',
        textAlign: 'left',
    }
}

const styleForContainer = () => {
    return {
        height: (props.panelHeight - settings.headerHeight - settings.footerHeight) + 'px',
        overflowY: 'scroll',
        position: 'relative',
        backgroundColor: '#111',
    }
}

const styleForColumn = (column, type) => {
    let extColumnWidth = 100;
    let sizeColumnWidth = 100;
    let datetimeColumnWidth = 200;

    if (column == 'filename') {
        let width = (props.panelWidth - sizeColumnWidth - extColumnWidth - datetimeColumnWidth - 30 - paddingLeftRight);
        return {
            width: width + 'px',
            minWidth: width + 'px',
            maxWidth: width + 'px',
            textAlign: 'left',
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            paddingLeft: paddingLeft + 'px',
            paddingRight: paddingRight + 'px',
            paddingTop: paddingTop + 'px',
            paddingBottom: paddingBottom + 'px',
        }
    }

    if (column == 'size') {
        return {
            width: sizeColumnWidth + 'px',
            minWidth: sizeColumnWidth + 'px',
            maxWidth: sizeColumnWidth + 'px',
            textAlign: 'right',
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            paddingLeft: paddingLeft + 'px',
            paddingRight: paddingRight + 'px',
            paddingTop: paddingTop + 'px',
            paddingBottom: paddingBottom + 'px',
        }
    }

    if (column == 'ext') {
        return {
            width: extColumnWidth + 'px',
            minWidth: extColumnWidth + 'px',
            maxWidth: extColumnWidth + 'px',
            textAlign: 'left',
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            paddingLeft: paddingLeft + 'px',
            paddingRight: paddingRight + 'px',
            paddingTop: paddingTop + 'px',
            paddingBottom: paddingBottom + 'px',
        }
    }

    if (column == 'datetime') {
        return {
            width: datetimeColumnWidth + 'px',
            minWidth: datetimeColumnWidth + 'px',
            maxWidth: datetimeColumnWidth + 'px',
            textAlign: 'left',
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            paddingLeft: paddingLeft + 'px',
            paddingRight: paddingRight + 'px',
            paddingTop: paddingTop + 'px',
            paddingBottom: paddingBottom + 'px',
        }
    }

    return {
    }
}


loadContent();

</script>

<template>
    <div style="display: block;" v-if="data.content != null">
        <div :style="styleForHeader()">
            {{ data.content.currentPath }}
        </div>
        <div style="position: relative;">
            <div class="scrollable-content" :id="tableContainerId()" :style="styleForContainer()">
                <table>
                    <thead>
                        <tr>
                            <th :style="styleForColumn('filename', 'header')">FileName</th>
                            <th :style="styleForColumn('ext', 'header')">Ext</th>
                            <th :style="styleForColumn('size', 'header')">Size</th>
                            <th :style="styleForColumn('datetime', 'header')">Date</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr :id="idForRow(index)" v-for="(file, index) in data.content.items" :key="index"
                            @click="onClickItem(index)" @dblclick="onDblClickItem(index)" :style="styleForItem(index)">
                            <td :style="styleForColumn('filename', 'header')" class="fileName">{{ file.displayName }}
                            </td>
                            <td :style="styleForColumn('ext', 'header')" class="fileName">{{ file.displayExt }}</td>
                            <td :style="styleForColumn('size', 'header')" class="fileSize">{{ file.size }}</td>
                            <td :style="styleForColumn('datetime', 'header')" class="fileSize">{{ file.datetime }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <div :style="styleForFooter()">
            <div style="font-size: 12px;">
                Full path: {{ currentItem().fullPath }}
            </div>
            <div style="font-size: 12px;">
                Link path: {{ currentItem().linkPath }}
            </div>
        </div>
    </div>
</template>

<style scoped>
div {
    font-family: 'Consolas', 'Courier New', Courier, monospace;
    font-size: 18px;
}

table {
    width: 100%;
    border-collapse: collapse;
}

th,
td {
    border: 1px solid #333;
    padding: 0px;
    text-align: left;
    user-select: none;
    height: 25px;
}

td {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    cursor: default;
}


thead {
    position: sticky;
    top: 0;
    z-index: 1;
    background-color: #000;
}

.scrollable-content {
    scrollbar-width: thin;
    scrollbar-color: #EEE #333333;
}

.scrollable-content::-webkit-scrollbar {
    width: 12px;
}

.scrollable-content::-webkit-scrollbar-track {
    background: #333333;
}

.scrollable-content::-webkit-scrollbar-thumb {
    background-color: #EEE;
    border-radius: 10px;
    border: 3px solid #333333;
}

.scrollable-content::-webkit-scrollbar-thumb:hover {
    background-color: #EEE;
}
</style>
