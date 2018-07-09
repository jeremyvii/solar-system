/**
 * Date derived from work by Singal, Tanmay & Singal, Ashok. (2009). in 
 * `Determining planetary positions in the sky for ~50 years to an accuracy of 
 * 1 degree with a calculator.`
 * 
 * @see        {@link: https://www.researchgate.net/publication/45878423_Determining_planetary_positions_in_the_sky_for_50_years_to_an_accuracy_of_1_degree_with_a_calculator}
 * @copyright  Jeremy Jackson 2018
 */

/**
 * Convert date string to UTC date object.
 * 
 * Similar to strtotime() in other languages
 * 
 * @param   {string}  date  Date string to convert to JavaScript Date object
 * 
 * @return  {Date}          String converted to date object
 */
const toUTC = (date) => {
  const result = new Date(Date.parse(date));
  result.setMinutes(result.getMinutes() - result.getTimezoneOffset());
  return result;
};

/**
 * Calculates difference between dates in days
 * 
 * Start date must be earlier than end date or the returned value will be an 
 * arbitrary negative number
 * 
 * @param   {string}  start  Start date must be an earlier date than end date
 * @param   {string}  end    End date
 * 
 * @return  {number}         Difference in days between two dates
 */
const dateDiff = (start, end) => {
  return (toUTC(end) - toUTC(start)) / (24 * 60 * 60 * 1000);
};

/**
 * Converts astronomical units (`AU`) to kilometers
 * 
 * @param   {number}  au  AU to be converted to kilometers
 * 
 * @return  {number}      AU converted to kilometers
 */
const auToKm = au => au * 149597870.7;

// Total days in an Earth Year
const YRS = 365.24;
// Distance to heliopause in astronomical units
const BOUNDS = 120;

/*
 * Represents a planets location and orbital properties
 */
class Planet {
  constructor(meanLongitude, period, angularSpeed, eccentricity) {
    /**
     * Location around the sun in degrees
     * 
     * @type  {number}
     */
    this.meanLongitude = meanLongitude;

    /**
     * Orbital period in days
     * 
     * @type  {number}
     */
    this.period = period;

    /**
     * Speed around in the sun (in degrees?)
     * 
     * @type  {number}
     */
    this.angularSpeed = angularSpeed;

    /**
     * Eccentricity of planets elipitical orbit
     * 
     * @type  {number}
     */
    this.eccentricity = eccentricity;
  }

  /**
   * Converts orbital period from days to years
   * 
   * @return  {number}
   */
  get years() {
    return this.period / YRS;
  }

  /**
   * Gets astronomical units from orbital period
   * 
   * @return  {number}
   */
  get au() {
    return Math.cbrt(Math.pow(this.years, 2));
  }
}

/**
 * Key/value sequence of planets in our wonderous solar system
 */
const planets = {
  mercury: new Planet(250.2, 87.969,  4.09235, 0.2056),
  venus:   new Planet(181.2, 224.701, 1.60213, 0.0068),
  earth:   new Planet(100.0, 365.256, 0.98561, 0.0167),
  mars:    new Planet(355.2, 686.98,  0.52403, 0.0934),
  jupiter: new Planet(34.3,  4332.59, 0.08309, 0.0485),
  saturn:  new Planet(50.1,  10759.2, 0.03346, 0.0555),
};

// Reference date that relates to our starting values
const REF = '1/1/2000';

/**
 * Finds where planet is located in degrees in our the sphere around the sun, 
 * in reference to `REF` constant
 * 
 * @param   {string}  planetStr  Name of planet to find location
 * @param   {string}  date       Date of planet's location in which return value 
 *                               is in reference too 
 * 
 * @return  {number}             Where planet is around the sun in degrees
 */
const planetAt = (planetStr, date) => {
  // Indicate if planet requested does not exist
  if (!planets.hasOwnProperty(planetStr)) 
    throw `Planet, ${planetAt}, does not exist`;
  // Find planet in based on name provided
  const planet = planets[planetStr];
  // Find difference in days between reference point on date requested
  const diff = dateDiff(REF, date);
  // Get planet's mean angle traversed by finding the product of planet's angular 
  // speed and difference in days
  const angleTraversed = planet.angularSpeed * diff;
  // Get the sum of planet's mean longitude and mean angle travsered. This will 
  // be the location of around the Sun in reference to reference date (`REF`).
  // The remainder of the sum divided by 360 degrees to ensure we have a value 
  // equal to or less than 360 degrees (or the degrees in a sphere)
  return (planet.meanLongitude + angleTraversed) % 360;
};

Array.from(Object.keys(planets)).forEach(planet => {
  // console.log(`heliopause: `);
  console.log(planet + ': ' + planetAt(planet, (function() {
    const today = new Date();
    let dd = today.getDate();
    let mm = today.getMonth() + 1;
    let yyyy = today.getFullYear();
    if (dd < 10) dd = '0' + dd;
    if (mm < 10) mm = '0' + mm;
    return `${mm}/${dd}/${yyyy}`;
  })()));
});
