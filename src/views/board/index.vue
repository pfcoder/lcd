<template>
  <div class="board-main">
    <el-dialog v-model="rebootSingleDialogVisible" :title="$t('reboot')" width="500" align-center>
      <span>{{ $t('sureToReboot') }}</span>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="rebootSingleDialogVisible = false">{{ $t('cancel') }}</el-button>
          <el-button type="primary" :loading="rebootLoading" @click="rebootSingle">
            {{ $t('confirm') }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog v-model="rebootMultipleDialogVisible" :title="$t('reboot')" width="500" align-center>
      <span>{{ $t('sureToReboot') }}</span>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="rebootMultipleDialogVisible = false">{{ $t('cancel') }}</el-button>
          <el-button type="primary" :loading="rebootLoading" @click="rebootMultiple">
            {{ $t('confirm') }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog v-model="configMultipleDialogVisible" :title="$t('config')" width="500" align-center>
      <div>{{ $t('selectedMachineWillBeUpdatedTo') }}</div>
      <div class="config-dialog-content" v-html="poolInfo"></div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="configMultipleDialogVisible = false">{{ $t('cancel') }}</el-button>
          <el-button type="primary" :loading="configLoading" @click="configMultiple">
            {{ $t('confirm') }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog v-model="configAllDialogVisible" :title="$t('config')" width="500" align-center>
      <div>{{ $t('allMachineWillBeUpdatedTo') }}</div>
      <div class="config-dialog-content" v-html="poolInfo"></div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="configAllDialogVisible = false">{{ $t('cancel') }}</el-button>
          <el-button type="primary" :loading="configLoading" @click="configAll">
            {{ $t('confirm') }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog v-model="machineChartsDialogVisible" :title="$t('runStatics')" width="70%"
      @close="isMachineRecordsLoading = true" align-center>
      <div class="charts-dialog-content">
        <machine-chart v-if="!isMachineRecordsLoading" :machine-records="machineRecords" />
      </div>
    </el-dialog>

    <el-drawer v-model="settingDrawer">
      <template #header>
        <h4>{{ $t('settings') }}</h4>
      </template>
      <template #default>
        <el-form :model="settingForm" label-width="auto">
          <el-form-item :label="$t('refreshInterval')">
            <el-select v-model="settingForm.refreshInterval" :placeholder="$t('plsSelectRefreshInterval')">
              <el-option label="3m" value="3" />
              <el-option label="5m" value="5" />
              <el-option label="10m" value="10" />
              <el-option label="30m" value="30" />
            </el-select>
          </el-form-item>
          <el-form-item :label="$t('connectTimeoutSetting')">
            <el-slider v-model="scanTimeout" :format-tooltip="formatScanTimeoutTooltip" :max="10" :min="2" />
          </el-form-item>
          <el-form-item :label="$t('language')">
            <el-select v-model="settingForm.language" :placeholder="$t('plsSelectLanguage')">
              <el-option label="中文" value="zh" />
              <el-option label="English" value="en" />
            </el-select>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="onSettingsConfirm">{{ $t('confirm') }}</el-button>
            <el-button @click="settingDrawer = false">{{ $t('cancel') }}</el-button>
          </el-form-item>
        </el-form>
      </template>
    </el-drawer>

    <div class="top-control">
      <div class="content">
        <div class="ip-area">
          <div class="ip-input">
            <el-select v-model="ip" filterable allow-create default-first-option :reserve-keyword="false"
              :placeholder="$t('inputOrSelectIp')">
              <el-option v-for="item in ipHistory" :key="item.value" :label="item.value" :value="item.value" />
            </el-select>
          </div>
          <div class="ip-scan">
            <el-button width="200" :icon="Search" @click="scan" type="primary"
              :disabled="scanning || ip.length === 0">{{
                buttonText }}</el-button>
          </div>
          <div class="machine-infos">
            <div class="machine-info">
              <div class="label">{{ $t('online') }}:</div>
              <div class="value">{{ methods.countMachines(true) }}</div>
            </div>
            <div class="machine-info">
              <div class="label">{{ $t('unknown') }}:</div>
              <div class="value">{{ methods.countMachines(false) }}</div>
            </div>
          </div>
        </div>
        <div class="action-area">
          <div class="button-area">
            <el-button :icon="Clock" @click="toggle_watch" :disabled="scanning" type="primary">{{ is_watching ?
              $t('cancelWatch')
              :
              $t('watch') }}</el-button>
            <el-button :icon="Refresh" @click="refresh_machine" :loading="is_refreshing"
              :disabled="is_refreshing || scanning || is_watching" type="primary">{{ $t('refresh') }}</el-button>
            <el-button :icon="Setting" @click="show_config_all" type="primary"
              :disabled="!(checked1 || checked2 || checked3)">{{ $t('configAll') }}</el-button>
            <el-button :icon="Setting" @click="show_config_selection" type="primary"
              :disabled="multipleSelection.length === 0 || scanning || !(checked1 || checked2 || checked3)">{{
                $t('configSelected') }}</el-button>
            <el-button :icon="SwitchButton" @click="show_reboot_selection" type="primary"
              :disabled="multipleSelection.length === 0 || scanning">{{ $t('rebootSelected') }}</el-button>
            <el-button :icon="Operation" @click="settingDrawer = true" type="primary">{{ $t('settings') }}</el-button>
          </div>
          <div class="pool-config-area">
            <div class="pool">
              <div class="check"><el-checkbox v-model="checked1" size="large"
                  :disabled="pool1.length === 0 || worker1.length === 0" /></div>
              <div class="label">{{ $t('pool1') }}:</div>
              <div class="input"><el-input v-model="pool1" :placeholder="$t('poolAddress')"></el-input></div>
              <div class="label">{{ $t('worker1') }}:</div>
              <div class="input"><el-input v-model="worker1" :placeholder="$t('workerAccount')" type='text'
                  autocomplete="off" @focus="methods.changeType"></el-input></div>
              <div class="label">{{ $t('pwd1') }}:</div>
              <div class="input"><el-input v-model="pwd1" :placeholder="$t('accountPassword')"></el-input></div>
            </div>
            <div class="pool">
              <div class="check"><el-checkbox v-model="checked2" size="large"
                  :disabled="pool2.length === 0 || worker2.length === 0" /></div>
              <div class="label">{{ $t('pool2') }}:</div>
              <div class="input"><el-input v-model="pool2" :placeholder="$t('poolAddress')"></el-input></div>
              <div class="label">{{ $t('worker2') }}:</div>
              <div class="input"><el-input v-model="worker2" :placeholder="$t('workerAccount')"></el-input></div>
              <div class="label">{{ $t('pwd2') }}:</div>
              <div class="input"><el-input v-model="pwd2" :placeholder="$t('accountPassword')"></el-input></div>
            </div>
            <div class="pool">
              <div class="check"><el-checkbox v-model="checked3" size="large"
                  :disabled="pool3.length === 0 || worker3.length === 0" /></div>
              <div class="label">{{ $t('pool3') }}:</div>
              <div class="input"><el-input v-model="pool3" :placeholder="$t('poolAddress')"></el-input></div>
              <div class="label">{{ $t('worker3') }}:</div>
              <div class="input"><el-input v-model="worker3" :placeholder="$t('workerAccount')"></el-input></div>
              <div class="label">{{ $t('pwd3') }}:</div>
              <div class="input"><el-input v-model="pwd3" :placeholder="$t('accountPassword')"></el-input></div>
            </div>
          </div>
        </div>

      </div>
    </div>
    <div class="machine-table">
      <el-table ref="multipleTableRef" @selection-change="handleSelectionChange" class="table-content" :height="800"
        border @row-click="handleRowClick" :row-style="{
          height: '16px'
        }" :data="machines" :default-sort="{ prop: 'updated_at', order: 'descending' }">
        <el-table-column type="selection" width="55" />
        <el-table-column prop="ip" width="130" label="IP" show-overflow-tooltip sortable
          align="center"></el-table-column>
        <el-table-column prop="status" width="90" :label="$t('status.label')" align="center">
          <template #default="{ row }">
            <template v-if="row.status === '在线'">
              <div style="color: green">{{ $t('status.online') }}</div>
            </template>
            <template v-else>
              <div style="color: red">{{ $t('status.offline') }}</div>
            </template>
          </template>
        </el-table-column>
        <el-table-column class="table-column" prop="machine_type" show-overflow-tooltip :label="$t('type')" with="100"
          sortable align="center"></el-table-column>
        <el-table-column class="table-column" prop="hash_real" width="120" show-overflow-tooltip sortable
          :sort-method="methods.hash_sort" :label="$t('hashReal')" align="center"></el-table-column>
        <el-table-column class="table-column" prop="hash_avg" width="120" show-overflow-tooltip sortable
          :sort-method="methods.hash_sort" :label="$t('hashAvg')" align="center"></el-table-column>
        <el-table-column class="table-column" prop="temp" width="120" show-overflow-tooltip :label="$t('temp')"
          align="center"></el-table-column>
        <el-table-column class="table-column" prop="mode" width="90" show-overflow-tooltip :label="$t('mode')"
          align="center">
          <template #default="{ row }">
            <template v-if="row.mode === '高功'">
              <div style="color: green">{{ $t('highMode') }}</div>
            </template>
            <template v-else>
              <div style="color: red">{{ $t('normalMode') }}</div>
            </template>
          </template>
        </el-table-column>
        <el-table-column class="table-column" prop="updated_at" width="120" :label="$t('updateTime')"
          show-overflow-tooltip sortable align="center">
          <template #default="{ row }">
            {{ methods.formatDate(row.updated_at) }}
          </template>
        </el-table-column>
        <el-table-column class="table-column" prop="elapsed" show-overflow-tooltip :label="$t('runTime')"
          align="center"></el-table-column>
        <el-table-column class="table-column" prop="pool1" show-overflow-tooltip :label="$t('pool1')" width="150"
          align="center"></el-table-column>
        <el-table-column class="table-column" width='100' prop="worker1" show-overflow-tooltip :label="$t('worker1')"
          sortable align="center"></el-table-column>
        <el-table-column class="table-column" prop="pool2" show-overflow-tooltip :label="$t('pool2')"
          align="center"></el-table-column>
        <el-table-column class="table-column" prop="worker2" show-overflow-tooltip :label="$t('worker2')"
          align="center"></el-table-column>
        <el-table-column class="table-column" :label="$t('operation.label')" fixed="right" width="150" align="center">
          <template #default="{ row }">
            <div class="operation">
              <el-button class="button" type="text" size="small" @click="handleShowCharts(row)">{{ $t('operation.chart')
                }}</el-button>
              <el-button class="button" type="text" size="small" @click="handleShowWeb(row)">{{ $t('operation.web')
                }}</el-button>
              <el-button class="button" type="text" size="small" @click="handleSingleReboot(row)">{{
                $t('operation.reboot')
                }}</el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup lang='ts'>
import { ref, reactive, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElTable, ElMessage } from 'element-plus';
import { Setting, Clock, Search, SwitchButton, Refresh, Operation } from '@element-plus/icons-vue';
import { open } from '@tauri-apps/api/shell';
import MachineChart from './components/chart.vue';
import { MachineInfo, MachineRecord } from '@/types';
import i18n from "@/lang";

const IP_HIS_KEY: string = 'lcd-ip-history';
const MACHINES_KEY: string = 'lcd-machines';
const CONFIG_POOL1_KEY: string = 'lcd-pool1';
const CONFIG_POOL2_KEY: string = 'lcd-pool2';
const CONFIG_POOL3_KEY: string = 'lcd-pool3';
const CONFIG_ACCOUNT1_KEY: string = 'lcd-account1';
const CONFIG_ACCOUNT2_KEY: string = 'lcd-account2';
const CONFIG_ACCOUNT3_KEY: string = 'lcd-account3';
const CONFIG_PWD1_KEY: string = 'lcd-pwd1';
const CONFIG_PWD2_KEY: string = 'lcd-pwd2';
const CONFIG_PWD3_KEY: string = 'lcd-pwd3';
const CONFIG_WATCHING_KEY: string = 'lcd-watching';
const CONFIG_REFRESH_KEY: string = 'lcd-refresh';
const CONFIG_LANG_KEY: string = 'lcd-lang';
const CONFIG_SCAN_TIMEOUT_KEY: string = 'lcd-scan-timeout';


interface PoolConfig {
  url: string;
  user: string;
  password: string;
}

interface IpHistory {
  value: string;
}

const machines = ref<MachineInfo[]>([]);
const machines_map = ref<Map<string, MachineInfo>>(new Map());
const ip = ref<string>("");
const scanning = ref<boolean>(false);
const buttonText = ref<string>("扫描");
const ipHistory = ref<IpHistory[]>([]);
const pool1 = ref<string>("");
const worker1 = ref<string>("");
const pwd1 = ref<string>("");
const pool2 = ref<string>("");
const worker2 = ref<string>("");
const pwd2 = ref<string>("");
const pool3 = ref<string>("");
const worker3 = ref<string>("");
const pwd3 = ref<string>("");
const checked1 = ref<boolean>(false);
const checked2 = ref<boolean>(false);
const checked3 = ref<boolean>(false);
const selectedRow = ref<MachineInfo>();
const is_watching = ref<boolean>(false);
const rebootLoading = ref<boolean>(false);
const configLoading = ref<boolean>(false);
const is_refreshing = ref<boolean>(false);

const settingDrawer = ref<boolean>(false);

const rebootSingleDialogVisible = ref<boolean>(false);
const rebootMultipleDialogVisible = ref<boolean>(false);
const configMultipleDialogVisible = ref<boolean>(false);
const configAllDialogVisible = ref<boolean>(false);
const machineChartsDialogVisible = ref<boolean>(false);
//const machineRecordsLoading = ref<boolean>(false);

const multipleTableRef = ref<InstanceType<typeof ElTable>>();
const multipleSelection = ref<MachineInfo[]>([]);

const machineRecords = ref<MachineRecord[]>([]);
const isMachineRecordsLoading = ref<boolean>(true);

const watchTaskId = ref<any>(0);
const scanTimeout = ref<number>(3);

let settingForm = reactive({
  refreshInterval: localStorage.getItem(CONFIG_REFRESH_KEY) || "3",
  language: localStorage.getItem(CONFIG_LANG_KEY) || "zh",
})

const handleSelectionChange = (val: MachineInfo[]) => {
  multipleSelection.value = val;
  //console.log("selection change:", val.length);
}

const poolInfo = computed(() => {
  let html = '<table>';

  if (checked1.value) {
    html += `<tr><td>矿池1:</td><td>${pool1.value}</td>`;
    html += `<td>矿工1:</td><td>${worker1.value}</td>`;
    html += `<td>密码1:</td><td>${pwd1.value}</td></tr>`;
    localStorage.setItem(CONFIG_POOL1_KEY, pool1.value);
    localStorage.setItem(CONFIG_ACCOUNT1_KEY, worker1.value);
    localStorage.setItem(CONFIG_PWD1_KEY, pwd1.value);
  }

  if (checked2.value) {
    html += `<tr><td>矿池2:</td><td>${pool2.value}</td>`;
    html += `<td>矿工2:</td><td>${worker2.value}</td>`;
    html += `<td>密码2:</td><td>${pwd2.value}</td></tr>`;
    localStorage.setItem(CONFIG_POOL2_KEY, pool2.value);
    localStorage.setItem(CONFIG_ACCOUNT2_KEY, worker2.value);
    localStorage.setItem(CONFIG_PWD2_KEY, pwd2.value);
  }

  if (checked3.value) {
    html += `<tr><td>矿池3:</td><td>${pool3.value}</td>`;
    html += `<td>矿工3:</td><td>${worker3.value}</td>`;
    html += `<td>密码3:</td><td>${pwd3.value}</td></tr>`;
    localStorage.setItem(CONFIG_POOL3_KEY, pool3.value);
    localStorage.setItem(CONFIG_ACCOUNT3_KEY, worker3.value);
    localStorage.setItem(CONFIG_PWD3_KEY, pwd3.value);
  }

  html += '</table>';
  return html;
});

onMounted(() => {
  // savedHistory is string array
  const savedHistory = JSON.parse(localStorage.getItem(IP_HIS_KEY) || '[]');
  console.log("history:", savedHistory);
  ipHistory.value = savedHistory.map((item: string) => {
    return { value: item };
  });

  machines_map.value = new Map(Object.entries(JSON.parse(localStorage.getItem(MACHINES_KEY) || '{}')));
  console.log("machines_map:", machines_map.value);

  machines.value = [];
  machines_map.value.forEach((v: MachineInfo, _k: string) => {
    machines.value.push(v);
  });

  // read settings
  //settingForm = JSON.parse(localStorage.getItem(CONFIG_SETTINGS_KEY) || '{}');

  // read pool config
  pool1.value = localStorage.getItem(CONFIG_POOL1_KEY) || "";
  worker1.value = localStorage.getItem(CONFIG_ACCOUNT1_KEY) || "";
  pwd1.value = localStorage.getItem(CONFIG_PWD1_KEY) || "";
  pool2.value = localStorage.getItem(CONFIG_POOL2_KEY) || "";
  worker2.value = localStorage.getItem(CONFIG_ACCOUNT2_KEY) || "";
  pwd2.value = localStorage.getItem(CONFIG_PWD2_KEY) || "";
  pool3.value = localStorage.getItem(CONFIG_POOL3_KEY) || "";
  worker3.value = localStorage.getItem(CONFIG_ACCOUNT3_KEY) || "";
  pwd3.value = localStorage.getItem(CONFIG_PWD3_KEY) || "";

  is_watching.value = localStorage.getItem(CONFIG_WATCHING_KEY) === '1';
  if (is_watching.value) {
    watch_machine();
  }
});

async function watch_machines() {
  let ips = machines.value.map((item: MachineInfo) => item.ip);
  let newMachines: MachineInfo[] = await invoke("watch_machines", { ips, timeout: scanTimeout.value });

  // update machine map according ip, if exist update, else add
  // update all machines status to offline
  machines.value.forEach((item: MachineInfo) => {
    item.status = "未知";
  });
  newMachines.forEach((item: MachineInfo) => {
    console.log("item:", item);
    if (!item.ip) {
      return;
    }
    item.status = "在线";
    item.updated_at = new Date().getTime();
    machines_map.value.set(item.ip, item);
    // find index and update
    let index = machines.value.findIndex((v: MachineInfo) => v.ip === item.ip);
    machines.value[index] = item;
  });

  // update store
  localStorage.setItem(MACHINES_KEY, JSON.stringify(Object.fromEntries(machines_map.value)));
}

const formatScanTimeoutTooltip = (val: number) => {
  return `${val}s`;
}

async function scan() {
  console.log("invoke scan");
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // set button to loading and disabled
  let savedHistory = JSON.parse(localStorage.getItem(IP_HIS_KEY) || '[]');
  if (!savedHistory.includes(ip.value)) {
    savedHistory.push(ip.value);
    localStorage.setItem(IP_HIS_KEY, JSON.stringify(savedHistory));
    ipHistory.value = savedHistory.map((item: string) => {
      return { value: item };
    });
  }

  scanning.value = true;
  buttonText.value = i18n.global.t('scanning') + " 0%";

  const scanPer = 64;
  const scanGroupCount = 256 / scanPer;

  // every 10 count scan
  for (let i = 0; i < scanGroupCount; i++) {
    //machines.value = machines.value.concat(await invoke("scan_machines", { ip: ip.value, offset: i * 8, count: 8 }));
    let newScans: MachineInfo[] = await invoke("scan_machines", { ip: ip.value, offset: i * scanPer, count: scanPer, timeout: scanTimeout.value });
    // update machine map according ip, if exist update, else add
    newScans.forEach((item: MachineInfo) => {
      console.log("item:", item);
      if (!item.ip) {
        return;
      }
      item.status = "在线";
      item.updated_at = new Date().getTime();
      if (!machines_map.value.has(item.ip)) {
        // new machine
        machines_map.value.set(item.ip, item);
        machines.value.push(item);
      } else {
        machines_map.value.set(item.ip, item);
        // find index and update
        let index = machines.value.findIndex((v: MachineInfo) => v.ip === item.ip);
        machines.value[index] = item;
      }
    });

    buttonText.value = i18n.global.t('scanning') + ` ${(i + 1) / scanGroupCount * 100}%`;
  }
  // update store
  localStorage.setItem(MACHINES_KEY, JSON.stringify(Object.fromEntries(machines_map.value)));

  scanning.value = false;
  buttonText.value = "扫描";
}

async function refresh_machine() {
  is_refreshing.value = true;
  await watch_machines();
  is_refreshing.value = false;
}

// function handleCellClick(row: MachineInfo, column: any, _cell: any, _event: any) {
//   console.log("cell click:", row, column);
//   if (column.property === 'ip' && row.ip) {
//     open(`http://${row.ip}`);
//   }
// }

function toggle_watch() {
  is_watching.value = !is_watching.value;
  localStorage.setItem(CONFIG_WATCHING_KEY, is_watching.value ? '1' : '0');
  if (is_watching.value) {
    ElMessage({
      message: '已开启定时刷新.',
      type: 'success',
    })
    watch_machine();
  } else {
    ElMessage({
      message: '已关闭定时刷新.',
      type: 'success',
    })
    clearInterval(watchTaskId.value);
  }
}

function watch_machine() {
  watchTaskId.value = setInterval(async () => {
    console.log("watch task");
    if (is_watching.value) {
      await watch_machines();
    } else {
      clearInterval(watchTaskId.value);
    }
  }, parseInt(settingForm.refreshInterval) * 60 * 1000);
}

function show_config_selection() {
  // check if user has choose
  configMultipleDialogVisible.value = true;
}

function show_config_all() {
  configAllDialogVisible.value = true;
}

function show_reboot_selection() {
  rebootMultipleDialogVisible.value = true;
}


function get_pool_config() {
  let pools: PoolConfig[] = [];
  if (checked1.value) {
    pools.push({ url: pool1.value, user: worker1.value, password: pwd1.value });
  }

  if (checked2.value) {
    pools.push({ url: pool2.value, user: worker2.value, password: pwd2.value });
  }

  if (checked3.value) {
    pools.push({ url: pool3.value, user: worker3.value, password: pwd3.value });
  }

  return pools;

}

async function configMultiple() {
  configLoading.value = true;
  console.log("config multiple:", multipleSelection.value);
  let pools = get_pool_config();
  let ips = multipleSelection.value.map((item: MachineInfo) => item.ip);
  let res = await invoke("config_machines", { ips, account: pools });
  console.log("config res:", res);
  configLoading.value = false;
  configMultipleDialogVisible.value = false;
}

function configAll() {
  configLoading.value = true;
  let pools = get_pool_config();
  let ips = machines.value.map((item: MachineInfo) => item.ip);
  invoke("config_machines", { ips, account: pools });
  configLoading.value = false;
  configAllDialogVisible.value = false;
}

function hash_sort(a: MachineInfo, b: MachineInfo) {
  // remove 
  let float_a = parseFloat(a.hash_real.split(" ")[0]);
  let float_b = parseFloat(b.hash_real.split(" ")[0]);
  return float_a - float_b;
}

function handleRowClick(row: MachineInfo, _event: any) {
  selectedRow.value = row;
  console.log("row click:", row)
}

function handleSingleReboot(_row: MachineInfo) {
  rebootSingleDialogVisible.value = true;
}

function handleShowCharts(row: any) {
  queryMachineRecords(row);
  machineChartsDialogVisible.value = true;
}

function handleShowWeb(row: MachineInfo) {
  open(`http://${row.ip}`);
}

async function rebootSingle(_event: any) {
  console.log("reboot single:", selectedRow.value);
  rebootLoading.value = true;
  // TODO: invoke reboot
  let res = await invoke("reboot_machines", { ips: [selectedRow?.value?.ip] });
  console.log("reboot res:", res);
  rebootSingleDialogVisible.value = false;
  rebootLoading.value = false;
  ElMessage({
    message: '已下发重启指令',
    type: 'success',
  })
}

async function rebootMultiple(_event: any) {
  // update button to loading
  rebootLoading.value = true;
  console.log("reboot multiple:", multipleSelection.value);
  let ips = multipleSelection.value.map((item: MachineInfo) => item.ip);
  let res = await invoke("reboot_machines", { ips });
  console.log("reboot res:", res);
  rebootMultipleDialogVisible.value = false;
  ElMessage({
    message: '已下发重启指令',
    type: 'success',
  })
  rebootLoading.value = false;
}

async function queryMachineRecords(row: MachineInfo) {
  console.log("query machine records:", row);
  isMachineRecordsLoading.value = true;
  // load from db
  let ip = row.ip;
  // from current date to 7 days ago
  let start = Math.round((new Date().getTime() - 7 * 24 * 60 * 60 * 1000) / 1000);
  let end = Math.round(new Date().getTime() / 1000);
  machineRecords.value = await invoke("query_ip_records", { ip, start, end });
  //console.log("records:", machineRecords.value);
  isMachineRecordsLoading.value = false;
}

function onSettingsConfirm() {
  console.log("onSettingsConfirm:", settingForm);
  // update store
  // get old settings compare
  let oldRefresh = localStorage.getItem(CONFIG_REFRESH_KEY);
  if (oldRefresh !== settingForm.refreshInterval) {
    localStorage.setItem(CONFIG_REFRESH_KEY, settingForm.refreshInterval);
    // update watch task
    if (is_watching.value) {
      clearInterval(watchTaskId.value);
      watch_machine();
    }
  }

  if (localStorage.getItem(CONFIG_LANG_KEY) !== settingForm.language) {
    localStorage.setItem(CONFIG_LANG_KEY, settingForm.language);
    i18n.global.locale.value = settingForm.language == 'zh' ? 'zh' : 'en';
  }

  if (localStorage.getItem(CONFIG_SCAN_TIMEOUT_KEY) !== scanTimeout.value.toString()) {
    localStorage.setItem(CONFIG_SCAN_TIMEOUT_KEY, scanTimeout.value.toString());
  }

  settingDrawer.value = false;
}

const methods = reactive({
  hash_sort,
  formatDate(date: number) {
    // format to MM-DD HH:mm:ss
    if (date === null || date === undefined || date === 0) {
      return "";
    }

    // convert from timestamp to date
    let dateConvert = new Date(date);

    return `${dateConvert.getMonth() + 1}-${dateConvert.getDate()} ${dateConvert.getHours()}:${dateConvert.getMinutes()}:${dateConvert.getSeconds()}`;
  },
  countMachines(online: boolean) {
    return machines.value.filter((item: MachineInfo) => item.status === (online ? "在线" : "未知")).length;
  },
  changeType(event: any) {
    console.log("onInput:", event);
    //event.target.autocapitalize = 'none';
    event.srcElement.type = 'text';
    event.target.autocapitalize = 'none';
  }
});


defineExpose({
  scan,
  methods
});
</script>

<style scoped lang='scss'>
.board-main {
  // flex from top to bottom
  flex-direction: column;
  height: 100%;
  display: flex;

  // top control area with fixed height
  .top-control {
    position: sticky;
    top: 0;
    height: 200px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    background-color: #f0f0f0;
    border-bottom: 1px solid #ebebeb;

    .title {
      font-size: 20px;
      font-weight: bold;
    }

    .content {
      display: flex;
      flex-direction: row;
      align-items: center;

      .ip-area {
        border: dashed 1px gray;
        height: 180px;
        width: 200px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        margin-left: 5px;

        .ip-input {
          padding: 10px;
          width: 180px;
        }

        .ip-scan {
          height: 30px;
          align-items: center;
        }

        .machine-infos {
          padding-top: 20px;
          display: flex;
          flex-direction: row;
          align-items: center;
          justify-content: space-between;

          .machine-info {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            margin: 10px;

            .label {
              font-size: 14px;
            }

            .value {
              font-size: 14px;
              font-weight: bold;
            }
          }
        }
      }

      .action-area {
        display: flex;
        flex-direction: column;
        margin-left: 20px;
        justify-content: space-between;

        .button-area {
          display: flex;
          flex-direction: row;
          padding-bottom: 10px;
        }

        .pool-config-area {
          display: flex;
          flex-direction: column;

          .pool {
            .check {
              width: 20px;
            }

            .label {
              width: 60px;
              font-size: 14px;
            }

            .input {
              width: 200px;
              padding-right: 10px;
            }

            display: flex;
            flex-direction: row;
            height: 25px;
            align-items: center;
            margin-top: 10px;
          }
        }
      }

      .graphic_area {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        .img {
          width: 300px;
          height: 100px;
        }
      }
    }
  }

  // remain space for table content
  .machine-table {
    flex: 1;
    padding: 10px;
    background-color: #f0f0f0;
    overflow-x: auto;

    .table-content {
      width: 2000px;
      font-size: 12px;
      font-weight: 700;

      .table-column {
        text-align: center;
        white-space: nowrap;
        overflow: ellipsis;
      }

      .td {
        overflow: ellipsis;
      }
    }

    .operation {
      display: flex;
      flex-direction: row;
      justify-content: space-around;
      align-items: center;

      .button {
        padding: 0;
      }
    }
  }

  .config-dialog-content {
    height: 200px;
  }

  .charts-dialog-content {
    height: 820px;
  }
}
</style>