import { useGql, useQuery } from '@notify-frontend/common';
import { getSdk } from './../operations.generated';

type SensorData = {
  sensor_id: string;
  store_name: string;
  location_name: string;
  sensor_name: string;
};

export const useColdChainSensors = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = ['COLDCHAIN_SENSORS'];

  return useQuery(cacheKeys, async () => {
    const sensorQuery =
      "SELECT sn.id as sensor_id, s.name as store_name,coalesce(l.description, '') as location_name, sn.name as sensor_name FROM SENSOR sn JOIN store s ON sn.storeid = s.id LEFT JOIN location l on sn.locationid = l.id WHERE sn.is_active = true ORDER BY 2,3,4 LIMIT 1000";
    const response = await sdk.getColdChainSensors({
      sqlQuery: sensorQuery,
      params: '{}',
    });

    const sensors: SensorData[] = JSON.parse(
      response?.runSqlQueryWithParameters
    );
    return sensors;
  });
};
