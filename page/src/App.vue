<template>
  <div class="container">
    <div class="header">
      <h1>NuttX-Rust Code Size Dashboard</h1>
      <p>View and compare code size metrics for different architectures and boards</p>
    </div>

    <div class="board-selector">
      <label for="board-select">Select Board: </label>
      <select id="board-select" v-model="selectedBoard" @change="loadBoardData">
        <option value="">Select a board...</option>
        <option v-for="board in boards" :key="board" :value="board">
          {{ board }}
        </option>
      </select>
    </div>

    <div v-if="loading" class="loading">Loading data...</div>

    <div v-if="error" class="error">
      {{ error }}
    </div>

    <div v-if="selectedBoard && !loading && !error">
      <div class="baseline-data">
        <h3>Baseline Data</h3>
        <div class="baseline-content">
          <div class="baseline-stats">
            <span>Text Size: {{ formatSize(currentData.builds[0]?.baseline.text || 0) }}</span>
            <span>Data Size: {{ formatSize(currentData.builds[0]?.baseline.data || 0) }}</span>
            <span>BSS Size: {{ formatSize(currentData.builds[0]?.baseline.bss || 0) }}</span>
            <span>Total Size: {{ formatSize(currentData.builds[0]?.baseline.total || 0) }}</span>
          </div>
          <div class="test-stats">
            <span class="test-passed">✅ Passed: {{ testStats.passed }}</span>
            <span class="test-failed">❌ Failed: {{ testStats.failed }}</span>
            <span class="test-skipped">⏭️ Skipped: {{ testStats.skipped }}</span>
          </div>
        </div>
      </div>

      <div class="filter-container">
        <label for="crate-filter">Filter by crate name: </label>
        <input
          id="crate-filter"
          type="text"
          v-model="crateFilter"
          placeholder="Enter crate name..."
          @input="filterTable"
        />
      </div>

      <div class="test-state-filters">
        <label>Filter by test state: </label>
        <button
          :class="['filter-btn', testStateFilter === 'all' ? 'active' : '']"
          @click="setTestStateFilter('all')"
        >
          All
        </button>
        <button
          :class="['filter-btn', testStateFilter === 'pass' ? 'active' : '']"
          @click="setTestStateFilter('pass')"
        >
          ✅ Pass
        </button>
        <button
          :class="['filter-btn', testStateFilter === 'fail' ? 'active' : '']"
          @click="setTestStateFilter('fail')"
        >
          ❌ Fail
        </button>
        <button
          :class="['filter-btn', testStateFilter === 'skip' ? 'active' : '']"
          @click="setTestStateFilter('skip')"
        >
          ⏭️ Skip
        </button>
      </div>

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
            <tr v-for="build in filteredBuilds" :key="build.crate_name">
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
                  @click="showOutputLog(build.crate_name, build.test.output)"
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
    </div>

    <!-- Output Log Modal -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Output Log - {{ currentLogCrate }}</h3>
          <button class="close-btn" @click="closeModal">&times;</button>
        </div>
        <div class="modal-body">
          <pre>{{ currentLogContent }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  import { ref, computed, onMounted } from 'vue';

  export default {
    name: 'App',
    setup() {
      const boards = ref([]);
      const selectedBoard = ref('');
      const currentData = ref({});
      const loading = ref(false);
      const error = ref('');
      const crateFilter = ref('');
      const testStateFilter = ref('all');
      const showModal = ref(false);
      const currentLogCrate = ref('');
      const currentLogContent = ref('');

      const loadBoardData = async () => {
        if (!selectedBoard.value) {
          currentData.value = {};
          return;
        }

        loading.value = true;
        error.value = '';

        try {
          console.log(`Loading data for board: ${selectedBoard.value}`);
          const response = await fetch(`/dist/${selectedBoard.value}`);
          console.log('Response status:', response.status, response.statusText);

          if (!response.ok) {
            throw new Error(`Failed to load data for ${selectedBoard.value}`);
          }
          const data = await response.json();
          console.log('Data loaded:', data);
          currentData.value = data;
        } catch (err) {
          console.error('Error loading board data:', err);
          error.value = `Error loading data: ${err.message}`;
          currentData.value = {};
        } finally {
          loading.value = false;
        }
      };

      const formatSize = size => {
        if (size === 0) return '0 B';
        if (size < 1024) return `${size} B`;
        if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
        return `${(size / (1024 * 1024)).toFixed(1)} MB`;
      };

      const getDiffClass = diff => {
        if (diff > 0) return 'size-positive';
        if (diff < 0) return 'size-negative';
        return '';
      };

      const filterTable = () => {
        // Filter logic is handled by the computed property
      };

      const setTestStateFilter = (state) => {
        testStateFilter.value = state;
      };

      const showOutputLog = (crateName, output) => {
        currentLogCrate.value = crateName;
        currentLogContent.value = output;
        showModal.value = true;
      };

      const closeModal = () => {
        showModal.value = false;
        currentLogCrate.value = '';
        currentLogContent.value = '';
      };

      const filteredBuilds = computed(() => {
        let builds = currentData.value.builds || [];

        // Apply crate name filter
        if (crateFilter.value) {
          const filter = crateFilter.value.toLowerCase();
          builds = builds.filter(build =>
            build.crate_name.toLowerCase().includes(filter)
          );
        }

        // Apply test state filter
        if (testStateFilter.value !== 'all') {
          builds = builds.filter(build => {
            if (!build.test) return false;
            
            if (testStateFilter.value === 'pass') {
              return build.test.success === true || build.test.success === 'pass';
            } else if (testStateFilter.value === 'fail') {
              return build.test.success === false || build.test.success === 'fail';
            } else if (testStateFilter.value === 'skip') {
              return build.test.success === 'skip';
            }
            return true;
          });
        }

        return builds;
      });

      const testStats = computed(() => {
        const builds = currentData.value.builds || [];
        let passed = 0;
        let failed = 0;
        let skipped = 0;

        builds.forEach(build => {
          if (build.test) {
            if (build.test.success === true || build.test.success === "pass") {
              passed++;
            } else if (build.test.success === false || build.test.success === "fail") {
              failed++;
            } else if (build.test.success === "skip") {
              skipped++;
            } else {
              skipped++;
            }
          } else {
            skipped++;
          }
        });

        return { passed, failed, skipped };
      });

      onMounted(async () => {
        try {
          const response = await fetch('/dist/boards.txt');
          if (response.ok) {
            const text = await response.text();
            boards.value = text
              .trim()
              .split('\n')
              .filter(line => line.trim());
          }
        } catch (err) {
          console.error('Failed to load boards list:', err);
          error.value = 'Failed to load available boards';
        }
      });

      return {
        boards,
        selectedBoard,
        currentData,
        loading,
        error,
        crateFilter,
        testStateFilter,
        showModal,
        currentLogCrate,
        currentLogContent,
        loadBoardData,
        formatSize,
        getDiffClass,
        filterTable,
        setTestStateFilter,
        showOutputLog,
        closeModal,
        filteredBuilds,
        testStats,
      };
    },
  };
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

  .filter-container {
    margin: 20px 0;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .filter-container input {
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
    min-width: 250px;
  }

  .filter-container input:focus {
    outline: none;
    border-color: #4caf50;
    box-shadow: 0 0 5px rgba(76, 175, 80, 0.2);
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

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background-color: white;
    border-radius: 8px;
    width: 90%;
    max-width: 800px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    border-bottom: 1px solid #eee;
  }

  .modal-header h3 {
    margin: 0;
    color: #333;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #666;
    padding: 0;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    color: #333;
  }

  .modal-body {
    padding: 20px;
    overflow-y: auto;
    flex: 1;
  }

  .modal-body pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family: 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.4;
    background-color: #f5f5f5;
    padding: 15px;
    border-radius: 4px;
    border: 1px solid #ddd;
    max-height: 60vh;
    overflow-y: auto;
  }

  .memory-pass {
    color: #4caf50;
    font-size: 16px;
  }

  .memory-leak {
    color: #f44336;
    font-weight: 500;
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

  .test-state-filters {
    margin: 20px 0;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .test-state-filters label {
    font-weight: 500;
    color: #333;
  }

  .filter-btn {
    padding: 8px 16px;
    border: 1px solid #ddd;
    border-radius: 4px;
    background-color: white;
    color: #666;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .filter-btn:hover {
    border-color: #2196f3;
    color: #2196f3;
  }

  .filter-btn.active {
    background-color: #2196f3;
    border-color: #2196f3;
    color: white;
  }

  .filter-btn.active:hover {
    background-color: #1976d2;
    border-color: #1976d2;
  }
</style>
