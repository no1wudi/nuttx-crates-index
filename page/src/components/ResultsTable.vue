<template>
  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>Crate Name</th>
          <th>Diff Text</th>
          <th>Diff Data</th>
          <th>Diff BSS</th>
          <th>Diff Total</th>
          <th>Execution Time</th>
          <th>Memory Leaked</th>
          <th>Test State</th>
          <th>Output Log</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="build in builds" :key="build.crate_name">
          <td>{{ build.crate_name }}</td>
          <td :class="getDiffClass(build.differences.text)">
            {{ formatSize(build.differences.text) }}
          </td>
          <td :class="getDiffClass(build.differences.data)">
            {{ formatSize(build.differences.data) }}
          </td>
          <td :class="getDiffClass(build.differences.bss)">
            {{ formatSize(build.differences.bss) }}
          </td>
          <td :class="getDiffClass(build.differences.total)">
            {{ formatSize(build.differences.total) }}
          </td>
          <td>{{ build.test.execution_time.toFixed(3) }} s</td>
          <td>
            <span v-if="build.test.success === false || build.test.success === 'fail'" class="memory-leak">❌</span>
            <span v-else-if="build.test.success === 'skip'" class="test-skip">⏭️</span>
            <span v-else-if="build.test.memory_leaked === 0" class="memory-pass">✅</span>
            <span v-else class="memory-leak">{{ formatSize(build.test.memory_leaked) }}</span>
          </td>
          <td>
            <span v-if="build.test.success === true || build.test.success === 'pass'" class="test-pass">✅ Pass</span>
            <span v-else-if="build.test.success === false || build.test.success === 'fail'" class="test-fail">❌ Fail</span>
            <span v-else class="test-skip">⏭️ Skip</span>
          </td>
          <td>
            <button
              v-if="build.test.output && build.test.output.trim()"
              @click="$emit('show-log', build.crate_name, build.test.output)"
              class="view-log-btn"
            >
              View Log
            </button>
            <span v-else class="no-log">No log</span>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  name: 'ResultsTable',
  props: {
    builds: {
      type: Array,
      required: true
    }
  },
  emits: ['show-log'],
  methods: {
    formatSize(size) {
      if (size === 0) return '0 B';
      if (size < 1024) return `${size} B`;
      if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
      return `${(size / (1024 * 1024)).toFixed(1)} MB`;
    },
    getDiffClass(diff) {
      if (diff > 0) return 'size-positive';
      if (diff < 0) return 'size-negative';
      return '';
    }
  }
}
</script>

<style scoped>
.table-container {
  overflow-x: auto;
  margin: 20px 0;
}

table {
  width: 100%;
  border-collapse: collapse;
  background-color: white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

th, td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

th {
  background-color: #f8f9fa;
  font-weight: 600;
  color: #333;
  position: sticky;
  top: 0;
  z-index: 10;
}

tr:hover {
  background-color: #f5f5f5;
}

.size-positive {
  color: #f44336;
  font-weight: 500;
}

.size-negative {
  color: #4caf50;
  font-weight: 500;
}

.size-positive {
  color: #f44336;
  font-weight: 500;
}

.memory-pass {
  color: #4caf50;
  font-size: 16px;
}

.memory-leak {
  color: #f44336;
  font-weight: 500;
}

.test-pass {
  color: #4caf50;
  font-weight: 500;
}

.test-fail {
  color: #f44336;
  font-weight: 500;
}

.test-skip {
  color: #ff9800;
  font-weight: 500;
}

.view-log-btn {
  padding: 6px 12px;
  background-color: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: background-color 0.3s;
}

.view-log-btn:hover {
  background-color: #1976d2;
}

.no-log {
  color: #999;
  font-size: 12px;
  font-style: italic;
}
</style>