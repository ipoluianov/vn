<script setup>
import { ref, reactive } from 'vue'

import { invoke } from "@tauri-apps/api/core";

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
        setCurrentItemIndex(data.content.current_item_index + 1);
    }
    if (event.key === 'ArrowUp') {
        event.preventDefault();
        setCurrentItemIndex(data.content.current_item_index - 1);
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
        setCurrentItemIndex(data.content.current_item_index - tableVisibleItemsCount());
    }
    if (event.key == 'PageDown') {
        event.preventDefault();
        setCurrentItemIndex(data.content.current_item_index + tableVisibleItemsCount());
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
    //await SetCurrentItemIndex(props.panelIndex, index);
    await invoke("set_current_item_index", { panelIndex: props.panelIndex, index: index });
    await loadContent();
}

const loadContent = async () => {
    console.log("loadContent", props.panelIndex);
    // greetMsg.value = await invoke("greet", { name: name.value });
    const content = await invoke("get_file_panel_content_as_json", { index: props.panelIndex });
    //const content = await get_file_panel_content_as_json(props.panelIndex);
    console.log("loadContent", content);
    data.content = JSON.parse(content);
    console.log("loadContent", data.content);
    setTimeout(() => {
        scrollToRow(idForRow(data.content.current_item_index));
    }, 10);

    /*data.content = {
        currentItemIndex: 0,
        currentPath: 'C:\\',
        items: [
            { display_name: 'file1.txt', display_ext: 'txt', size: '1KB', datetime: '2021-01-01 12:00:00', full_path: 'C:\\file1.txt', link_path: 'C:\\file1.txt' },
            { display_name: 'file2.txt', display_ext: 'txt', size: '2KB', datetime: '2021-01-01 12:00:00', full_path: 'C:\\file2.txt', link_path: 'C:\\file2.txt' },
            { display_name: 'file3.txt', display_ext: 'txt', size: '3KB', datetime: '2021-01-01 12:00:00', full_path: 'C:\\file3.txt', link_path: 'C:\\file3.txt' },
    ]
        }*/
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
    return data.content.items[data.content.current_item_index];
}

const styleForItem = (index) => {
    if (props.isActive == true) {
        return {
            backgroundColor: index === data.content.current_item_index ? '#444' : '#00000000'
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

const getCurrentItemFullPath = () => {
    if (data == null) {
        return '';
    }
    if (data.content == null) {
        return '';
    }
    if (data.content.items.length == 0) {
        return '';
    }
    if (data.content.current_item_index < 0) {
        return '';
    }
    if (data.content.current_item_index >= data.content.items.length) {
        return '';
    }
    return data.content.items[data.content.current_item_index].full_path;
}

const getCurrentItemLinkPath = () => {
    if (data == null) {
        return '';
    }
    if (data.content == null) {
        return '';
    }
    if (data.content.items.length == 0) {
        return '';
    }
    if (data.content.current_item_index < 0) {
        return '';
    }
    if (data.content.current_item_index >= data.content.items.length) {
        return '';
    }
    return data.content.items[data.content.current_item_index].link_path;
}

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
                            <td :style="styleForColumn('filename', 'header')" class="fileName">{{ file.display_name }}
                            </td>
                            <td :style="styleForColumn('ext', 'header')" class="fileName">{{ file.display_ext }}</td>
                            <td :style="styleForColumn('size', 'header')" class="fileSize">{{ file.size }}</td>
                            <td :style="styleForColumn('datetime', 'header')" class="fileSize">{{ file.datetime }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <div :style="styleForFooter()">
            <div style="font-size: 12px;">
                Full path: {{ getCurrentItemFullPath() }}
            </div>
            <div style="font-size: 12px;">
                Link path: {{ getCurrentItemLinkPath() }}
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
