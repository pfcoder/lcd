export interface MachineInfo {
    ip: string;
    status: string;
    machine_type: string;
    hash_real: string;
    hash_avg: string;
    temp: string;
    mode: string;
    elapsed: string;
    pool1: string;
    worker1: string;
    pool2: string;
    worker2: string;
    updated_at: number;
  }

  export interface MachineRecord {
    id: number;
    ip: string,
    machine_type: string,
    work_mode: number,
    hash_real: number,
    hash_avg: number,
    temp_0: number,
    temp_1: number,
    temp_2: number,
    power: number,
    create_time: number,
  }