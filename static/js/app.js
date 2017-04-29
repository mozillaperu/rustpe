$(document).ready(function(){

  $.get('http://rustpe.org/events', function(data) {
    $.each(data, function(k, v) {
      eventTemplate(v.name, convertDate(v.date), v.about, v.slide_event);
    })
  });

  function eventTemplate(name, date, about, slide) {
    return $('#events').append('<h3>'+name+'</h3>' +
           '<h4>'+date+'</h4>' +
           '<p>'+about+'</p>' +
           '<p><a class="white" href="'+slide+'" target="_blank">Presentación</a></p>');
  }

  function convertDate(date) {
    return moment(date).format('DD/MM/YYYY hh:mm A');
  }

});
