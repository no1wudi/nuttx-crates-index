<template>
  <div class="baseline-data">
    <h3>Baseline Data</h3>
    <div class="baseline-content">
      <div class="baseline-stats">
        <span>Text Size: {{ formatSize(baseline.text || 0) }}</span>
        <span>Data Size: {{ formatSize(baseline.data || 0) }}</span>
        <span>BSS Size: {{ formatSize(baseline.bss || 0) }}</span>
        <span>Total Size: {{ formatSize(baseline.total || 0) }}</span>
      </div>
      <div class="test-stats">
        <span class="test-passed">✅ Passed: {{ testStats.passed }}</span>
        <span class="test-failed">❌ Failed: {{ testStats.failed }}</span>
        <span class="test-skipped">⏭️ Skipped: {{ testStats.skipped }}</span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'BaselineData',
  props: {
    baseline: {
      type: Object,
      required: true
    },
    testStats: {
      type: Object,
      required: true
    }
  },
  methods: {
    formatSize(size) {
      if (size === 0) return '0 B';
      if (size < 1024) return `${size} B`;
      if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
      return `${(size / (1024 * 1024)).toFixed(1)} MB`;
    }
  }
}
</script>

<style scoped>
.baseline-data {
  margin: 20px 0;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 5px;
}

.baseline-data h3 {
  margin: 0 0 10px 0;
  color: #333;
}

.baseline-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 20px;
}

.baseline-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.baseline-stats span {
  font-weight: 500;
  color: #666;
}

.test-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.test-passed {
  color: #4caf50;
  font-weight: 500;
}

.test-failed {
  color: #f44336;
  font-weight: 500;
}

.test-skipped {
  color: #ff9800;
  font-weight: 500;
}
</style>