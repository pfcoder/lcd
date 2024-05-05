<template>
  <div class="chart" ref="chartRefHash"></div>
  <div class="chart" ref="chartRefPower"></div>
  <div class="chart" ref="chartRefHpRatio"></div>
  <div class="chart" ref="chartRefTemp"></div>
</template>

<script setup lang='ts'>
import { ref, onMounted } from 'vue'
import * as echarts from 'echarts';
import { MachineRecord } from '@/types';
import i18n from '@/lang';

const props = defineProps<{
  machineRecords: MachineRecord[];
}>()

const chartRefHash = ref(null);
const chartRefPower = ref(null);
const chartRefHpRatio = ref(null);
const chartRefTemp = ref(null);

onMounted(() => {
  const chartHash = echarts.init(chartRefHash.value);
  const chartPower = echarts.init(chartRefPower.value);
  const chartHp = echarts.init(chartRefHpRatio.value);
  const chartTemp = echarts.init(chartRefTemp.value);
  // xAxis is recent 7 days, every day 24 hours
  // convert machineRecords[i].create_time to MM-DD HH:mm:ss
  let xAxisData = [];
  let yHashReal = [];
  let yHashAvg = [];
  let yPower = [];
  let yHp = [];
  let yTemp1 = [];
  let yTemp2 = [];
  let yTemp3 = [];

  for (const item of props.machineRecords) {
    // format create_time to MM-DD HH:mm:ss
    let date = new Date(item.create_time * 1000); // 假设 create_time 是一个以秒为单位的时间戳

    let MM_DD = date.toLocaleDateString('en-us', { month: '2-digit', day: '2-digit' });
    let HH_mm_ss = date.toLocaleTimeString('en-us', { hour12: false });

    let formattedDate = `${MM_DD} ${HH_mm_ss}`;

    xAxisData.push(formattedDate);
    yHashReal.push(item.hash_real);
    yHashAvg.push(item.hash_avg);
    yPower.push(item.power);
    if (item.power > 0) {
      // format to 3 decimal places
      yHp.push((item.hash_real / item.power).toFixed(3));
    } else {
      yHp.push(0);
    }
    yTemp1.push(item.temp_0);
    yTemp2.push(item.temp_1);
    yTemp3.push(item.temp_2);
  }

  chartHash.setOption({
    title: {
      text: i18n.global.t('hashStatics')
    },
    tooltip: {
      trigger: 'axis'
    },
    legend: {
      data: [i18n.global.t('hashReal'), i18n.global.t('hashAvg')]
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: xAxisData
    },
    yAxis: {
      type: 'value'
    },
    series: [
      {
        name: i18n.global.t('hashReal'),
        type: 'line',
        //stack: 'Total',
        data: yHashReal
      },
      {
        name: i18n.global.t('hashAvg'),
        type: 'line',
        //stack: 'Total',
        data: yHashAvg
      }
    ]
  });

  chartPower.setOption({
    title: {
      text: i18n.global.t('powerStatics')
    },
    tooltip: {
      trigger: 'axis'
    },
    legend: {
      data: [i18n.global.t('power')]
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: xAxisData
    },
    yAxis: {
      type: 'value'
    },
    series: [
      {
        name: i18n.global.t('power'),
        type: 'line',
        //stack: 'Total',
        data: yPower,
        color: '#f00'
      }
    ]
  });

  chartHp.setOption({
    title: {
      text: i18n.global.t('hashPowerStatics'),
    },
    tooltip: {
      trigger: 'axis'
    },
    legend: {
      data: [i18n.global.t('hashPower')]
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: xAxisData
    },
    yAxis: {
      type: 'value'
    },
    series: [
      {
        name: i18n.global.t('hashPower'),
        type: 'line',
        //stack: 'Total',
        data: yHp,
        color: '#1ee'
      }
    ]
  });

  chartTemp.setOption({
    title: {
      text: i18n.global.t('tempStatics')
    },
    tooltip: {
      trigger: 'axis'
    },
    legend: {
      data: [i18n.global.t('temp1'), i18n.global.t('temp2'), i18n.global.t('temp3')]
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: xAxisData
    },
    yAxis: {
      type: 'value'
    },
    series: [
      {
        name: i18n.global.t('temp1'),
        type: 'line',
        //stack: 'Total',
        data: yTemp1,
        color: '#339999'
      },
      {
        name: i18n.global.t('temp2'),
        type: 'line',
        //stack: 'Total',
        data: yTemp2,
        color: '#339999'
      },
      {
        name: i18n.global.t('temp3'),
        type: 'line',
        //stack: 'Total',
        data: yTemp3,
        color: '#339999'
      }
    ]
  });
});
</script>

<style scoped lang='scss'>
.chart {
  border: 1px dashed #ccc;
  border-radius: 5px;
  margin-top: 5px;
  height: 200px;
  width: 100%;
}
</style>