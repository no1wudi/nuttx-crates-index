<template>
  <div class="arch-selector">
    <label for="arch-select">Select Arch: </label>
    <select id="arch-select" :value="selectedArch" @change="$emit('change', $event.target.value)">
      <option value="">Select an architecture...</option>
      <option v-for="arch in archs" :key="arch" :value="arch">
        {{ arch }}
      </option>
    </select>
  </div>
</template>

<script>
export default {
  name: 'ArchitectureSelector',
  props: {
    archs: {
      type: Array,
      required: true
    },
    selectedArch: {
      type: String,
      required: true
    }
  },
  emits: ['change']
}
</script>

<style scoped>
.arch-selector {
  margin-bottom: 20px;
  text-align: center;
}

.arch-selector label {
  font-weight: 500;
  color: #333;
  margin-right: 10px;
}

.arch-selector select {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  min-width: 200px;
  background-color: white;
}

.arch-selector select:focus {
  outline: none;
  border-color: #4caf50;
  box-shadow: 0 0 5px rgba(76, 175, 80, 0.2);
}
</style>