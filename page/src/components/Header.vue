<template>
  <div class="header">
    <h1>NuttX-Rust Code Size Dashboard</h1>
    <p>View and compare code size metrics for different architectures</p>
  </div>
</template>

<script>
export default {
  name: 'Header'
}
</script>

<style scoped>
.header {
  text-align: center;
  margin-bottom: 30px;
  padding: 30px 20px;
  background: linear-gradient(45deg, #667eea 0%, #764ba2 25%, #f093fb 50%, #4facfe 75%, #667eea 100%);
  background-size: 400% 400%;
  border-radius: 10px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  animation: gradientShift 15s ease infinite;
}

@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

.header h1 {
  color: white;
  margin-bottom: 10px;
  font-size: 2.5em;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
}

.header p {
  color: rgba(255, 255, 255, 0.9);
  font-size: 1.2em;
  font-weight: 500;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.3);
}
</style>