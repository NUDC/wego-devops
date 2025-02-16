<template>
  <div ref="ce"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { EditorView, basicSetup } from 'codemirror';
import { StreamLanguage } from '@codemirror/language';
import { shell } from '@codemirror/legacy-modes/mode/shell';

const value = defineModel<string>(); // 绑定 v-model
const ce = ref<HTMLElement | null>(null);
const view = ref<EditorView | null>(null);

onMounted(() => {
  if (!ce.value) return;

  view.value = new EditorView({
    doc: value.value || '',
    parent: ce.value,
    extensions: [
      basicSetup,
      StreamLanguage.define(shell),
      EditorView.updateListener.of(v => {
        const newValue = v.state.doc.toString();
        if (newValue !== value.value) {
          value.value = newValue;
        }
      }),
    ],
  });
});

// 监听 v-model 变化，实时同步到编辑器
watch(value, newVal => {
  if (view.value && newVal !== view.value.state.doc.toString()) {
    view.value.dispatch({
      changes: { from: 0, to: view.value.state.doc.length, insert: newVal || '' },
    });
  }
});

onUnmounted(() => {
  view.value?.destroy();
  view.value = null;
});
</script>
