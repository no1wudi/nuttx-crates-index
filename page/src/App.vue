<template>
  <div class="container">
    <Header />
    
    <ArchitectureSelector
      :archs="archs"
      :selected-arch="selectedArch"
      @change="onArchChange"
    />

    <div v-if="loading" class="loading">Loading data...</div>

    <div v-if="error" class="error">
      {{ error }}
    </div>

    <div v-if="selectedArch && !loading && !error">
      <BaselineData
        :baseline="currentData.builds[0]?.baseline || {}"
        :test-stats="testStats"
      />

      <Filters
        v-model:crate-filter="crateFilter"
        v-model:test-state-filter="testStateFilter"
      />

      <ResultsTable
        :builds="filteredBuilds"
        @show-log="showOutputLog"
      />
    </div>

    <OutputLogModal
      :show="showModal"
      :crate-name="currentLogCrate"
      :content="currentLogContent"
      @close="closeModal"
    />
  </div>
</template>

<script>
  import { ref, computed, onMounted } from 'vue';
  import Header from './components/Header.vue';
  import ArchitectureSelector from './components/ArchitectureSelector.vue';
  import BaselineData from './components/BaselineData.vue';
  import Filters from './components/Filters.vue';
  import ResultsTable from './components/ResultsTable.vue';
  import OutputLogModal from './components/OutputLogModal.vue';

  export default {
    name: 'App',
    components: {
      Header,
      ArchitectureSelector,
      BaselineData,
      Filters,
      ResultsTable,
      OutputLogModal
    },
    setup() {
      const archs = ref([]);
      const selectedArch = ref('');
      const currentData = ref({});
      const loading = ref(false);
      const error = ref('');
      const crateFilter = ref('');
      const testStateFilter = ref('all');
      const showModal = ref(false);
      const currentLogCrate = ref('');
      const currentLogContent = ref('');

      const loadArchData = async () => {
        if (!selectedArch.value) {
          currentData.value = {};
          return;
        }

        loading.value = true;
        error.value = '';

        try {
          console.log(`Loading data for arch: ${selectedArch.value}`);
          const response = await fetch(`./${selectedArch.value}`);
          console.log('Response status:', response.status, response.statusText);

          if (!response.ok) {
            throw new Error(`Failed to load data for ${selectedArch.value}`);
          }
          const data = await response.json();
          console.log('Data loaded:', data);
          currentData.value = data;
        } catch (err) {
          console.error('Error loading arch data:', err);
          error.value = `Error loading data: ${err.message}`;
          currentData.value = {};
        } finally {
          loading.value = false;
        }
      };

      const onArchChange = (arch) => {
        selectedArch.value = arch;
        loadArchData();
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
          const response = await fetch('./boards.txt');
          if (response.ok) {
            const text = await response.text();
            archs.value = text
              .trim()
              .split('\n')
              .filter(line => line.trim());
          }
        } catch (err) {
          console.error('Failed to load archs list:', err);
          error.value = 'Failed to load available architectures';
        }
      });

      return {
        archs,
        selectedArch,
        currentData,
        loading,
        error,
        crateFilter,
        testStateFilter,
        showModal,
        currentLogCrate,
        currentLogContent,
        onArchChange,
        showOutputLog,
        closeModal,
        filteredBuilds,
        testStats,
      };
    },
  };
</script>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  font-family: Arial, sans-serif;
}

.loading {
  text-align: center;
  padding: 20px;
  font-size: 18px;
  color: #666;
}

.error {
  background-color: #ffebee;
  color: #c62828;
  padding: 15px;
  border-radius: 5px;
  margin: 20px 0;
  border-left: 4px solid #c62828;
}
</style>
